mod models;
mod schema;

use actix_web::{get, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use actix_files as fs;
use markdown::to_html;
use dotenvy::dotenv;
use std::env;
use crate::models::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::sqlite::Sqlite;

fn establish_connection() -> SqliteConnection {
    dotenv().expect(".env file not found");

    let url = ::std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    SqliteConnection::establish(&url).unwrap_or_else(|_| panic!("Couldn't connect to {}", url))
}

#[get("ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}

struct GetPostsByDateArgs {
    date: String,
}

fn get_posts_by_date(args: GetPostsByDateArgs) {
}

fn wrap_with_html_scaffold(content: String) -> String {
    format!("<html><title>Gerard's VIdeas</title><link rel=\"stylesheet\" type=\"text/css\" href=\"/static/styles.css\">{}</html>", content)
}

fn post_to_html(post: &Post) -> String {
    String::from(
        format!("<div class=\"post-card\">{}<pre class=\"post-card__date\">{}</pre></div>",
                post.title,
                post.created_at
        )
    )
}

fn posts_to_html(posts: Vec<Post>) -> String {
    let posts_html = posts.iter().map(|post| post_to_html(post)).collect::<Vec<String>>().join("\n");
    wrap_with_html_scaffold(posts_html)
}



#[get("/")]
async fn list_by_date() -> impl Responder {
    let conn = &mut establish_connection();
    let posts = post::table
        .load(conn)
        .expect("Error loading posts");

    HttpResponse::Ok().body(posts_to_html(posts))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .wrap(Logger::default())
            .service(ping)
            //.service(index)
            .service(list_by_date)
    })
    .bind(("127.0.0.1", 8444))?
    .run()
    .await
}

