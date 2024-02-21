use actix_web::{get, App, HttpServer, HttpResponse, Responder, middleware::Logger, web};
use actix_files as fs;
use dotenvy::dotenv;
use crate::utils::wrap_with_html_scaffold;
use crate::db::models::*;
use askama_actix::{Template, TemplateToResponse};

mod db;
mod filters;
mod posts;
mod utils;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    posts: &'a [Post],
}

#[derive(Template)]
#[template(path = "posts/show.html")]
struct ShowTemplate<'a> {
    post: &'a Post,
}

#[get("ping")]
#[doc(hidden)]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}

#[get("/")]
#[doc(hidden)]
async fn index() -> impl Responder {
    let posts_ = posts::all();
    IndexTemplate { posts: &posts_ }.to_response()
}

#[get("/entry/{id}")]
#[doc(hidden)]
async fn show_entry(path: web::Path<i32>) -> impl Responder {
    if let Ok(post_) = posts::one(path.into_inner()) {
        ShowTemplate { post: &post_ }.to_response()
    } else {
        // TODO: Add a proper 404 page
        HttpResponse::NotFound().body(wrap_with_html_scaffold("<h1>NOT FOUND</h1>"))
    }
}

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
            .service(show_entry)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

