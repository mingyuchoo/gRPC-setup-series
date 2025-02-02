use serde::Serialize;
use warp::Filter;

mod proto_bookstore;
use proto_bookstore::book_store_client::BookStoreClient;
use proto_bookstore::GetBookRequest;


#[derive(Serialize)]
struct BookResponse {
    id:     String,
    name:   String,
    author: String,
    year:   i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the warp filter for the GET /book route
    let book_route =
        warp::path!("book" / String).and_then(|id: String| async move {
            println!("Received request URL: /book/{}", id);
            fetch_book(id).await
        });

    // Start the warp server
    warp::serve(book_route).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

// Function to fetch book data from the proto server
async fn fetch_book(id: String) -> Result<impl warp::Reply, warp::Rejection> {
    let mut client = BookStoreClient::connect("http://[::1]:50051")
        .await
        .map_err(|e| {
            eprintln!("Failed to connect to the server: {}", e);
            warp::reject::not_found()
        })?;
    let request = tonic::Request::new(GetBookRequest {
        id: id.clone()
    });

    let response = client.get_book(request).await.map_err(|e| {
        eprintln!("Failed to get book: {}", e);
        warp::reject::not_found()
    })?;

    let book = response.into_inner();
    println!("Received response from Proto server: {:?}", book);

    let book_response = BookResponse {
        id:     book.id,
        name:   book.name,
        author: book.author,
        year:   book.year,
    };

    let json = serde_json::to_string(&book_response).map_err(|e| {
        eprintln!("Failed to serialize response: {}", e);
        warp::reject::not_found()
    })?;

    Ok(warp::reply::html(json))
}
