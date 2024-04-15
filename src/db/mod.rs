use diesel::sqlite::SqliteConnection;
use diesel::Connection;
use log::info;

pub mod models;
pub mod schema;

pub fn connect_db() -> SqliteConnection {
    let url = ::std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    info!("Connecting to {}", &url);
    SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Couldn't connect to {}", url))
}
