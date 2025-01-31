extern crate chrono;
extern crate dotenv;
extern crate postgres;

pub mod user {
    tonic::include_proto!("user");
}

use tonic::transport::Server;
use user::user_service_server::UserServiceServer;

extern crate console;
extern crate uuid;

use crate::service::User;
use console::Style;

mod db_connection;
mod service;

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
