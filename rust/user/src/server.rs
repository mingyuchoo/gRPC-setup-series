use crate::service::User;
use console::Style;
use tonic::transport::Server;
use user::user_service_server::UserServiceServer;

mod db_connection;
mod service;

pub mod user {
    tonic::include_proto!("user");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let user = User::default();
    let blue = Style::new().blue();

    println!("\nRust gRPC Server ready at {}", blue.apply_to(addr));

    Server::builder()
        .add_service(UserServiceServer::new(user))
        .serve(addr)
        .await?;

    Ok(())
}
