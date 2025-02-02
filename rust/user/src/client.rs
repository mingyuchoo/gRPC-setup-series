use warp::Filter;

mod db_connection;
mod service;

pub mod user {
    tonic::include_proto!("user");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let route = warp::path!("user" / "list")
        .map(|| {
            warp::reply::json(&"Hello, World!")
        });
        
    // Start the warp server
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
