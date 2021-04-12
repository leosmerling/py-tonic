use pyo3::prelude::*;
use pyo3_asyncio::into_future;

use crate::model::{HelloRequest, HelloResponse};

pub async fn exec_event(payload: HelloRequest) -> PyResult<HelloResponse> {
    let fut = Python::with_gil(|py| {
        let target = PyModule::import(py, "app.event").expect("err m");
        let m = PyModule::import(py, "engine.steps").expect("err m");
        let func = m.getattr("run_event").expect("err func");
        let coro = func.call1((target, payload,)).expect("error coro");
        let fut = into_future(coro).expect("error fut");
        fut
    });
    let out = fut.await.expect("err out");
    let res = Python::with_gil(|py| out.extract::<HelloResponse>(py).expect("err res"));

    println!("py-tonic from rust: Payload: - Result: {:?}", res);
    Ok(res)
}

pub fn init_module(_py: Python, m: &PyModule) -> () {
    m.add_class::<HelloRequest>().expect("err add_class");
    m.add_class::<HelloResponse>().expect("err add_class");
}
