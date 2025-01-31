use dotenv::dotenv;
use postgres::{Connection, TlsMode};
use std::env;

pub fn establish_connection() -> Connection {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Connection::connect(database_url, TlsMode::None).unwrap()
}
