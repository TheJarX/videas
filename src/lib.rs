// TOOD: check if this is a proper way to do this
pub mod db;
pub mod filters;
pub mod posts;
pub mod utils;
pub mod routes;

// TODO: move to its own file
pub mod templates {
    use askama_actix::Template;
    use super::db::models::*;
    use super::filters;

    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct IndexTemplate<'a> {
        pub posts: &'a [Post],
    }

    #[derive(Template)]
    #[template(path = "posts/show.html")]
    pub struct ShowTemplate<'a> {
        pub post: &'a Post,
    }
}
