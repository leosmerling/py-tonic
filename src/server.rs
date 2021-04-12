use tonic::{Request, Response, Status};
use log::{info};

use crate::model::hw::greeter_server::{Greeter};
use crate::model::hw::{HelloResponse, HelloRequest};

use crate::event::exec_event;

#[derive(Default, Debug, serde::Serialize)]
pub struct MyGreeter { }

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloResponse>, Status> { // Return an instance of type HelloReply
        info!("Python say_hello.request: {:?}", request.get_ref());
        let payload = request.into_inner();
        let response = exec_event(payload.into()).await;
        Ok(Response::new(response.expect("err response").into())) // Send back our formatted greeting
    }
}
