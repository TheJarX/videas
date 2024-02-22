use actix_web::{get, HttpResponse, Responder, web};
use log::{info};
use super::templates::*;
use super::posts;
use askama_actix::TemplateToResponse;

// Due sqlite limitations this is the defaul value of posts' slug
const NO_SLUG: &str = "NO_SLUG";


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

#[get("/post/{slug}")]
#[doc(hidden)]
pub async fn show_post(path: web::Path<String>) -> impl Responder {
    let slug = path.into_inner();

    if slug.eq(NO_SLUG) {
        return log_and_render_not_found("Default slug was found!");
    }
    if let Ok(post_) = posts::one_by_slug(&slug) {
        ShowTemplate { post: &post_ }.to_response()
    } else {
        log_and_render_not_found("Not found")
    }
}

fn log_and_render_not_found(message: &str) -> HttpResponse {
    info!("{}; rendering 404", message);
    NotFoundTemplate {}.to_response()
}

