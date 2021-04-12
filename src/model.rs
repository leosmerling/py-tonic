use pyo3::prelude::*;

pub mod hw {
    tonic::include_proto!("hw"); // The string specified here must match the proto package name
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct HelloRequest {
    #[pyo3(get)]
    name: String
}

#[pymethods]
impl HelloRequest {
    #[new]
    fn new(name: &str) -> Self {
        HelloRequest { name: name.into() }
    }
}

impl From<hw::HelloRequest> for HelloRequest {
    fn from(other: hw::HelloRequest) -> Self {
        HelloRequest { name: other.name }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct HelloResponse {
    #[pyo3(get, set)]
    message: String
}

#[pymethods]
impl HelloResponse {
    #[new]
    fn new(message: &str) -> Self {
        HelloResponse { message: message.into() }
    }
}

impl From<HelloResponse> for hw::HelloResponse {
    fn from(other: HelloResponse) -> Self {
        hw::HelloResponse { message: other.message }
    }
}
