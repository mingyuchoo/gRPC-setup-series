use dotenv::dotenv;
use postgres::{Client, NoTls};
use std::env;

pub fn establish_connection() -> Client {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Client::connect(&database_url, NoTls).unwrap()
}
