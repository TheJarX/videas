use crate::db;
use crate::db::models::*;
use crate::db::schema::*;
use diesel::prelude::*;

fn post_to_html(post: &Post) -> String {
    String::from(
        format!("<div class=\"post-card\"><a href=\"/posts/{}\">{}<pre class=\"post-card__date\">{}</pre></div>",
                post.id,
                post.title,
                post.created_at
        )
    )
}

fn posts_to_html(posts: Vec<Post>) -> String {
    posts.iter().map(|post| post_to_html(post)).collect::<Vec<String>>().join("\n")
}

pub fn all_as_html() -> String {
    posts_to_html(all())
}

pub fn one_as_html(post: &Post) -> String {
    post_to_html(post)
}

pub fn all() -> Vec<Post> {
    let conn = &mut db::connect_db();
    post::table.load(conn).expect("Error loading posts")
}

pub fn one(id: i32) -> Post {
    let conn = &mut db::connect_db();
    post::table.select(id).first()
}
