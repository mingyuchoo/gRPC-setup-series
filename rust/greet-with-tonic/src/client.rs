use serde::Serialize;
use warp::Filter;

mod proto_greeter;
use proto_greeter::greeting_client::GreetingClient;
use proto_greeter::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the wrap filter for the GET /greet route
    let greeter_route =
        warp::path!("greet" / String).and_then(|name: String| async move {
            println!("Received request URL: /greet/{}", name);
            fetch_greet(name).await
        });

    // Start the warp server
    warp::serve(greeter_route).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

// Function to fetch greet message from the proto server
async fn fetch_greet(
    name: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut client = GreetingClient::connect("http://[::1]:50051")
        .await
        .map_err(|e| {
            eprintln!("Failed to connect to the server: {}", e);
            warp::reject::not_found()
        })?;
    let request = tonic::Request::new(HelloRequest {
        name: name.clone()
    });
    let response = client.say_hello(request).await.map_err(|e| {
        eprintln!("Failed to get message: {}", e);
        warp::reject::not_found()
    })?;

    let message = response.into_inner();
    println!("Received response from Proto server: {:?}", message);

    let message_response = MessageResponse {
        message: message.message,
    };

    let json = serde_json::to_string(&message_response).map_err(|e| {
        eprintln!("Failed to serialize response: {}", e);
        warp::reject::not_found()
    })?;

    Ok(warp::reply::html(json))
}

#[derive(Serialize)]
struct MessageResponse {
    message: String,
}
