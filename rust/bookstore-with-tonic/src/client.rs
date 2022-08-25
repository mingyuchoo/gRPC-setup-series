
use my_book_store::book_store_client::BookStoreClient;
use my_book_store::GetBookRequest;

pub mod my_book_store {
    tonic::include_proto!("bookstore");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BookStoreClient::connect("http://[::1]:50051").await?;
    let request = tonic::Request::new(GetBookRequest {
        id: "1".into(),
    });
    let response = client.get_book(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}