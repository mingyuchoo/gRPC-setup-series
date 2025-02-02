use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod proto_greeter;
use proto_greeter::greeting_server::{Greeting, GreetingServer};
use proto_greeter::{HelloRequest, HelloResponse};

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

#[tonic::async_trait]
impl Greeting for MyGreeting {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = proto_greeter::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(response))
    }
}

#[derive(Debug, Default)]
pub struct MyGreeting {}
