use diesel::Connection;
use diesel::sqlite::SqliteConnection;

pub mod schema;
pub mod models;

pub fn connect_db() -> SqliteConnection {
    let url = ::std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Couldn't connect to {}", url))
}
