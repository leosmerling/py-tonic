#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Extra {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub num: u64,
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
