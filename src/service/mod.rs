use std::time::Instant;

use pyo3::prelude::*;
use simple_process_stats::ProcessStats;

mod model;
pub use model::hw::{Extra, HelloRequest, HelloResponse};

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
    for i in 0..10 * batch_size {
        let request = HelloRequest {
            name: "nobody".into(),
            extra: Some(Extra {
                text: "extra text".into(),
                num: 142
            })
        };
        let result = say_hello(request);
        if i % batch_size == 0 {
            let elapsed = start.elapsed();
            println!("{} ==============", i);
            println!("RUST pyres: {:?}", result);
            println!("time: {:?} lat/req: {:?} rust mem: {:?} python mem: {}",
                elapsed, elapsed / batch_size, process_stats.memory_usage_bytes / 1024, memory_footprint()
            );
            start = Instant::now();
        }
    }
}
