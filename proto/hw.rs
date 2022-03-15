#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extra {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    ///uint64 num = 2;
    #[prost(double, tag = "3")]
    pub numx: f64,
    #[prost(enumeration = "Status", tag = "4")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub extra: ::core::option::Option<Extra>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloResponse {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub extra: ::core::option::Option<Extra>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Unknown = 0,
    New = 1,
    Old = 2,
}
