use std::env;
use metered::{metered, Throughput, HitCount, ResponseTime};

use serde_yaml;
use log::{info};

use hw::greeter_client::GreeterClient;
use hw::{HelloRequest};

mod hw {
    tonic::include_proto!("hw");
}

#[derive(Default, Debug, serde::Serialize)]
pub struct Service {
    metrics: Metrics,
}


#[metered(registry = Metrics)]
impl Service {
    #[measure([HitCount, Throughput, ResponseTime])]
    pub async fn say_hello(&self, client: &mut GreeterClient<tonic::transport::Channel>) -> Result<(), Box<dyn std::error::Error>> {        
        let request = tonic::Request::new(HelloRequest {
            name: "Leo".into(),
        });
        let response = client.say_hello(request).await?;
        let reply = response.get_ref();
        //println!("RESPONSE={:?}", response);
        info!("Got reply: {}", reply.message);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    let service = Service::default();
    if args.contains(&"python".to_string()) {
        reqs_python(&service).await?;
        let stats = serde_yaml::to_string(&service).unwrap();
        println!("PYTHON: CLIENT STATS {}", stats);
    }
    Ok(())
}

async fn reqs_python(service: &Service) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let n = 50000u32;
    for _ in 0u32..n {
        service.say_hello(&mut client).await?;
    }
    Ok(())
}
