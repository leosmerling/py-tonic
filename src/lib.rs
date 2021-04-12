mod event;
mod model;
mod server;

use std::time::Duration;
use log::{info, error};

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3_asyncio::tokio::into_coroutine;
use tokio::time::sleep;
use tonic::transport::Server;

use model::hw::greeter_server::{GreeterServer};

use server::MyGreeter;


#[pyo3_asyncio::tokio::main]
async fn main() -> Result<(), PyErr> {
    env_logger::init();

    let addr = "[::1]:50051".parse().expect("err addr");
    let greeter = MyGreeter::default();

    match Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await {
            Ok(_) => {
                Ok(())
            },
            Err(x) => {
                error!("Interrumpted");
                Err(PyErr::new::<PyAny, String>(format!("{:?}", x)))
            }
        }

}

#[pyfunction]
fn run() -> PyResult<()> {
    info!("Rust gRPC server starting...");
    Ok(main())
}

#[pyfunction]
fn info() -> PyResult<String> {
    Ok("py-tonic 0.1!".into())
} 

async fn _sleep(secs: u64) {
    info!("About to sleep {} seconds...", secs);
    sleep(Duration::from_secs(secs)).await;
}

#[pyfunction]
fn asleep(py: Python, secs: &PyAny) -> PyResult<PyObject> {
    let secs = secs.extract()?;
    into_coroutine(py, async move {
        _sleep(secs).await;
        Python::with_gil(|py| Ok(py.None()))
    })
}

#[pyfunction]
fn init_event(py: Python, event_name: &str) -> PyResult<()> {
    info!("Initializing python module {}...", event_name);
    let m = PyModule::import(py, event_name)?;
    event::init_module(py, m);
    Ok(())
}

#[pymodule]
fn py_tonic(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init_event, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(info, m)?)?;
    m.add_function(wrap_pyfunction!(asleep, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info() {
        assert_eq!(info(), "py-tonic 0.1!".to_string());
    }
}
