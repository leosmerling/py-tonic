use pyo3::prelude::*;

pub mod hw {
    #[derive(Clone, PartialEq, prost::Message)]
    #[::pyo3::prelude::pyclass]
    pub struct Extra {
        #[prost(string, tag = "1")]
        #[pyo3(get, set)]
        pub text: ::prost::alloc::string::String,
        #[prost(uint64, tag = "2")]
        #[pyo3(get, set)]
        pub num: u64,
    }

    #[derive(Clone, PartialEq, prost::Message)]
    #[::pyo3::prelude::pyclass]
    pub struct HelloRequest {
        #[prost(string, tag = "1")]
        #[pyo3(get, set)]
        pub name: prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        #[pyo3(get, set)]
        pub extra: ::core::option::Option<Extra>,    
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    #[::pyo3::prelude::pyclass]
    pub struct HelloResponse {
        #[prost(string, tag = "1")]
        #[pyo3(get, set)]
        pub message: prost::alloc::string::String,
        #[prost(message, optional, tag = "2")]
        #[pyo3(get, set)]
        pub extra: ::core::option::Option<Extra>,    
    }

    #[::pyo3::prelude::pymethods]
    impl Extra {
        #[new]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[::pyo3::prelude::pymethods]
    impl HelloRequest {
        #[new]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[::pyo3::prelude::pymethods]
    impl HelloResponse {
        #[new]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[::pyo3::prelude::pyproto]
    impl ::pyo3::class::basic::PyObjectProtocol for HelloRequest {
        fn __repr__(&self) -> ::pyo3::PyResult<String> {
            Ok(format!("{:?}", self))
        }
        fn __str__(&self) -> ::pyo3::PyResult<String> {
            Ok(format!("{:#?}", self))
        }
    }

    #[::pyo3::prelude::pyproto]
    impl ::pyo3::class::basic::PyObjectProtocol for HelloResponse {
        fn __repr__(&self) -> ::pyo3::PyResult<String> {
            Ok(format!("{:?}", self))
        }
        fn __str__(&self) -> ::pyo3::PyResult<String> {
            Ok(format!("{:#?}", self))
        }
    }
}

use hw::{Extra, HelloRequest, HelloResponse};

pub fn setup() -> PyResult<()> {
    Python::with_gil(|py| {
        let module = py.import("greeter.hw")?;
        module.add_class::<Extra>()?;
        module.add_class::<HelloRequest>()?;
        module.add_class::<HelloResponse>()?;
        Ok(())
    })
}
