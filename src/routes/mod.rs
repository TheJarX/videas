use actix_web::{get, HttpResponse, Responder, web};
use super::utils::wrap_with_html_scaffold;
use super::templates::*;
use super::posts;
use askama_actix::TemplateToResponse;



#[get("ping")]
#[doc(hidden)]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}

#[get("/")]
#[doc(hidden)]
pub async fn index() -> impl Responder {
    let posts_ = posts::all();
    IndexTemplate { posts: &posts_ }.to_response()
}

#[get("/entry/{id}")]
#[doc(hidden)]
pub async fn show_entry(path: web::Path<i32>) -> impl Responder {
    if let Ok(post_) = posts::one(path.into_inner()) {
        ShowTemplate { post: &post_ }.to_response()
    } else {
        // TODO: Add a proper 404 page
        HttpResponse::NotFound().body(wrap_with_html_scaffold("<h1>NOT FOUND</h1>"))
    }
}

