use tonic::transport::Server;
use tonic::{Request, Response, Status};

mod proto_bookstore;
use proto_bookstore::book_store_server::{BookStore, BookStoreServer};
use proto_bookstore::{GetBookRequest, GetBookResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let bookstore = MyBookStore::default();

    println!("BookStore server listening on {}", addr);

    Server::builder()
        .add_service(BookStoreServer::new(bookstore))
        .serve(addr)
        .await?;

    Ok(())
}

#[derive(Debug, Default)]
pub struct MyBookStore {}

#[tonic::async_trait]
impl BookStore for MyBookStore {
    async fn get_book(
        &self,
        request: Request<GetBookRequest>,
    ) -> Result<Response<GetBookResponse>, Status> {
        println!("Request from: {:?}", request.remote_addr());

        let response = proto_bookstore::GetBookResponse {
            id:     request.into_inner().id,
            author: "Peter".to_owned(),
            name:   "Zero to One".to_owned(),
            year:   2025,
        };

        println!("Response to: {:?}", response);

        Ok(Response::new(response))
    }
}
