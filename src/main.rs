use actix_web::{get, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use actix_files as fs;
// use markdown::to_html;
use dotenvy::dotenv;
use std::env;

mod posts;
mod db;


#[get("ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}

fn wrap_with_html_scaffold(content: String) -> String {
    format!("<html><title>Gerard's VIdeas</title><link rel=\"stylesheet\" type=\"text/css\" href=\"/static/styles.css\">{}</html>", content)
}

#[get("/")]
async fn index() -> impl Responder {
    let posts_ = posts::all_as_html();
    HttpResponse::Ok().body(wrap_with_html_scaffold(posts_))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(ping)
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

