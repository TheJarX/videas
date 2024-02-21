use actix_web::{get, App, HttpServer, HttpResponse, Responder, middleware::Logger, web};
use actix_files as fs;
// use markdown::to_html;
use dotenvy::dotenv;
use crate::utils::wrap_with_html_scaffold;

mod posts;
mod utils;
mod db;


#[get("ping")]
#[doc(hidden)]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong!")
}

fn wrap_with_profile(content: &str) -> String {
    let profile="
    <div class=\"flex index-container\">
        <div class=\"index-container__left\">
            <div>
            <img src=\"https://avatars.githubusercontent.com/u/35399391\" >
            </div>
            <p class=\"justified\">Developer with expertise in Full-Stack, games development, and bash automation (I code scripts because I'm lazy lol).</p>
            <p class=\"justified\">I'm refreshing my website (including the blog), and you're lucky to see this new version (WIP). I'm planning to make the project open source since it has a couple of interesting things; need to finish some stuff first.</p>
        </div>
        <div class=\"index-container__right\">
            @@content@@
        </div>
    </div>
    ".replace("@@content@@", content);

    wrap_with_html_scaffold(&profile)
}
#[get("/")]
#[doc(hidden)]
async fn index() -> impl Responder {
    let posts_ = posts::show_all_as_html();
    let container = format!("<div class=\"flex flex-col items-center \">{}</div>", posts_);
    HttpResponse::Ok().body(wrap_with_profile(&container))
}

#[get("/entry/{id}")]
#[doc(hidden)]
async fn show_entry(path: web::Path<i32>) -> impl Responder {
    if let Ok(post_) = posts::show_one_as_html(path.into_inner()) {
        HttpResponse::Ok().body(wrap_with_html_scaffold(&post_))
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

