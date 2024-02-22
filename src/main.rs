use actix_web::{App, HttpServer, middleware::Logger};
use actix_files as fs;
use dotenvy::dotenv;
use videas::routes::*;

#[actix_web::main]
#[doc(hidden)]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(fs::Files::new("/static", "./static"))
            .service(ping)
            .service(index)
            .service(show_post)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

