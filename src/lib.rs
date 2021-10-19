use pyo3::prelude::*;

pub mod hw {
    tonic::include_proto!("hw");
}

use hw::{HelloRequest, HelloResponse};

pub fn setup() -> PyResult<()> {
    Python::with_gil(|py| {
        let module = py.import("greeter.hw")?;
        module.add_class::<HelloRequest>()?;
        module.add_class::<HelloResponse>()?;
        Ok(())
    })
}

pub fn say_hello(request: HelloRequest) -> HelloResponse {
    Python::with_gil(|py| {
        let module = py.import("greeter.say_hello").expect("err");
        let func = module.getattr("execute").expect("err");
        let pyres = func.call1((request,)).expect("err");
        println!("RUST pyres: {:?}", pyres);
        pyres.extract().expect("err")
    })
}
