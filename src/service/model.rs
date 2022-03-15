pub mod hw {
    use pyo3::prelude::*;
    use pyo3::PyResult;
    use pyo3::class::basic::PyObjectProtocol;
    use pyo3::class::basic::CompareOp;
    use prost::{Enumeration, Message};
    use prost::alloc::string::String;

    #[derive(Clone, PartialEq, prost::Message)]
    #[pyclass]
    pub struct Extra {
        #[prost(string, tag = "1")]
        #[pyo3(get)]
        pub text: String,
        // #[prost(uint64, tag = "2")]
        // #[pyo3(get)]
        // pub num: u64,
        #[prost(double, optional, tag = "3")]
        #[pyo3(get)]
        pub numx: Option<f64>,
        #[prost(enumeration = "Status", tag = "4")]
        #[pyo3(get)]
        pub status: i32,
    }

    #[derive(Clone, PartialEq, Message)]
    #[pyclass]
    pub struct HelloRequest {
        #[prost(string, tag = "1")]
        #[pyo3(get)]
        pub name: String,
        #[prost(message, optional, tag = "2")]
        #[pyo3(get)]
        pub extra: Option<Extra>,    
    }
    #[derive(Clone, PartialEq, Message)]
    #[pyclass]
    pub struct HelloResponse {
        #[prost(string, tag = "1")]
        #[pyo3(get)]
        pub message: String,
        #[prost(message, optional, tag = "2")]
        #[pyo3(get)]
        pub extra: Option<Extra>,    
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    #[repr(i32)]
    pub enum Status {
        Unknown = 0,
        New = 1,
        Old = 2,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[pyclass(name="Status")]
    pub struct StatusType {
        #[pyo3(get)]
        value: i32,
    }

    #[pymethods]
    impl StatusType {
        #[classattr]
        const UNKNOWN: StatusType = StatusType { value: 0 };
        #[classattr]
        const NEW: StatusType = StatusType { value: 1 };
        #[classattr]
        const OLD: StatusType = StatusType { value: 2 };
    }

    impl From<Status> for StatusType {
        fn from(value: Status) -> Self {
            match value {
                Status::Unknown => Self::UNKNOWN,
                Status::New => Self::NEW,
                Status::Old => Self::OLD
            }
        }
    }

    impl From<&StatusType> for Status {
        fn from(value: &StatusType) -> Self {
            match value {
                StatusType { value: 1 } => Status::New,
                StatusType { value: 2 } => Status::Old,
                _ => Status::Unknown
            }
        }
    }

    impl From<i32> for StatusType {
        fn from(value: i32) -> Self {
            Self { value }
        }
    }

    impl From<StatusType> for i32 {
        fn from(value: StatusType) -> i32 {
            value.value
        }
    }

    #[pyproto]
    impl PyObjectProtocol for StatusType {
        fn __repr__(&self) -> PyResult<String> {
            let status: Status = self.into();
            Ok(format!("{:?}", status))
        }
        fn __str__(&self) -> PyResult<String> {
            Ok(format!("{:#?}", self))
        }

        fn __richcmp__(&self, other: i32, op: CompareOp) -> PyResult<bool> {
            //println!("__richcmp__ {:?}=={:?}", self, other);
            match op {
                CompareOp::Eq => Ok(self.value == other),
                _ => Ok(false),
            }
        }

        fn __hash__(&self) -> PyResult<i32> {
            Ok(self.value)
        }
    }

    #[pymethods]
    impl Extra {
        #[new]
        pub fn new(
            text: String,
            numx: Option<f64>,
            status: StatusType,
        ) -> Self {
            Self {
                text,
                numx,
                status: status.value,
            }
        }
    }

    #[pymethods]
    impl HelloRequest {
        #[new]
        pub fn new(
            name: String,
            extra: Option<Extra>
        ) -> Self {
            Self {
                name,
                extra
            }
        }    
    }

    #[pymethods]
    impl HelloResponse {
        #[new]
        pub fn new(
            message: String,
            extra: Option<Extra>
        ) -> Self {
            Self {
                message,
                extra
            }
        }
    }

    #[pyproto]
    impl PyObjectProtocol for Extra {
        fn __repr__(&self) -> PyResult<String> {
            Ok(format!("{:?}", self))
        }
        fn __str__(&self) -> PyResult<String> {
            Ok(format!("{:#?}", self))
        }
    }

    #[pyproto]
    impl PyObjectProtocol for HelloRequest {
        fn __repr__(&self) -> PyResult<String> {
            Ok(format!("{:?}", self))
        }
        fn __str__(&self) -> PyResult<String> {
            Ok(format!("{:#?}", self))
        }
    }

    #[pyproto]
    impl PyObjectProtocol for HelloResponse {
        fn __repr__(&self) -> PyResult<String> {
            Ok(format!("{:?}", self))
        }
        fn __str__(&self) -> PyResult<String> {
            Ok(format!("{:#?}", self))
        }
    }
}

use pyo3::prelude::*;
use hw::{Extra, HelloRequest, HelloResponse, StatusType};

pub fn setup() -> PyResult<()> {
    Python::with_gil(|py| {
        let module = py.import("greeter.hw")?;
        module.add_class::<StatusType>()?;
        module.add_class::<Extra>()?;
        module.add_class::<HelloRequest>()?;
        module.add_class::<HelloResponse>()?;
        Ok(())
    })
}
