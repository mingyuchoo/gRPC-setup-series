use hello_world::greeting_server::{Greeting, GreetingServer};
use hello_world::{HelloRequest, HelloResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod hello_world {
    tonic::include_proto!("greeter");
}

#[derive(Debug, Default)]
pub struct MyGreeting {}

#[tonic::async_trait]
impl Greeting for MyGreeting {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = hello_world::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeting::default();

    Server::builder()
        .add_service(GreetingServer::new(greeter))
        .serve(addr)
        .await?;
    Ok(())
}
