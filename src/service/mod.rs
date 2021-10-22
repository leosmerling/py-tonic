use std::time::Instant;
use rand::random;

use pyo3::prelude::*;
use simple_process_stats::ProcessStats;

mod model;
pub use model::hw::{Extra, HelloRequest, HelloResponse, Status};

pub fn say_hello(request: HelloRequest) -> HelloResponse {
    Python::with_gil(|py| {
        let module = py.import("greeter.say_hello").expect("err");
        let func = module.getattr("execute").expect("err");
        let pyres = func.call1((request,)).expect("err");
        // println!("RUST pyres: {:?}", pyres);
        pyres.extract().expect("err")
    })
}


fn memory_footprint() -> usize {
    Python::with_gil(|py| {
        let module = py.import("greeter.say_hello").expect("err");
        let func = module.getattr("memory_footprint").expect("err");
        let pyres = func.call0().expect("err");
        // println!("RUST pyres: {:?}", pyres);
        pyres.extract().expect("err")
    })
}

#[tokio::main]
async fn main() {
    let process_stats = ProcessStats::get().await.expect("failed stats");

    model::setup().expect("Err setup");

    let batch_size = 1000000;
    let mut start = Instant::now();
    for i in 0..100 * batch_size {
        let random_status = match random::<f32>() {
            x if x > 0.5 => Status::New.into(),
            _ => Status::Old.into(),
        };
        let request = HelloRequest::new(
            "nobody".into(),
            Some(Extra::new(
                "extra text".into(),
                Some(142.42),
                random_status,
            )),
        );
        if i % batch_size == 0 {
            println!("{} ===========================================================", i);
            println!("RUST request: {:?}", request);
        }
        let result = say_hello(request);
        if i % batch_size == 0 {
            let elapsed = start.elapsed();
            println!("RUST pyres: {:?}", result);
            println!("time: {:?} lat/req: {:?} rust mem: {:?} python mem: {}",
                elapsed, elapsed / batch_size, process_stats.memory_usage_bytes / 1024, memory_footprint()
            );
            start = Instant::now();
        }
    }
}
