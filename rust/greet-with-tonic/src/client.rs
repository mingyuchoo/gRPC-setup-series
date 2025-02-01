mod hello_world;
use hello_world::greeting_client::GreetingClient;
use hello_world::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreetingClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });
    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
