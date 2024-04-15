use actix_files as fs;
use actix_web::{middleware::Logger, App, HttpServer};
use dotenvy::dotenv;
use videas::routes::*;

#[actix_web::main]
#[doc(hidden)]
async fn main() -> std::io::Result<()> {
    // We need this to load the .env file  only in development.
    // It's better to use other ways to load the environment variables in production.
    dotenv().ok();
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
