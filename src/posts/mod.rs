//! Contains abstracions related to Posts; mainly db queries
//!
//! # Considerations
//! - A "Post Card" is basically the element shown in the index
//! - A "Show Post" is the actual `Post` to be shown
//! - [BEM](https://getbem.com/) will be used as styles convention

use crate::db;
use crate::db::models::*;
use crate::db::schema::*;
use diesel::prelude::*;
use markdown;


/// Method that will generate the HTML for a Post to be shown in the index.
///
/// This method will return an string containing the markup for a preview of an article
/// aiming to be shown in the index. Hence only the `id`, `title` and the creation date(`created_at`) of a post will be used.
///
/// # Examples
/// ```rust
/// post_to_card_html(&one(my_id))
/// ```
/// Will produce something like (actual output may vary):
/// ```html
/// <div class="post-card">
///   <a href=":id">:title</a>
///   <pre class="post-card__date">:created_at</pre>
/// </div>
/// ```
fn post_to_card_html(post: &Post) -> String {
    String::from(
        format!("<div class=\"post-card\">
                    <a href=\"/entry/{}\">
                    {}<pre class=\"post-card__date\">{}</pre>
                    </a>
                </div>",
                post.id,
                post.title,
                post.created_at
        )
    )
}

/// Joins the result of calling [`post_to_card_html`] of every element of `posts`
///
/// [`post_to_card_html`]: fn.post_to_card_html.html
fn posts_to_cards_html(posts: Vec<Post>) -> String {
    posts.iter().map(|post| post_to_card_html(post)).collect::<Vec<String>>().join("\n")
}

/// Return the result of calling [`posts_to_cards_html`] using the result of [`all`] as argument
///
/// [`posts_to_cards_html`]: fn.posts_to_cards_html.html
/// [`all`]: fn.all.html
pub fn show_all_as_html() -> String {
    posts_to_cards_html(all())
}

/// Parse a `Post` content as html
pub fn show_one_as_html(id: i32) -> QueryResult<String> {
    let post_ = one(id)?;
    let content = format!("
        <div class=\"flex flex-col content-center items-center post-container\">
            <div class=\"post-container__content\">
            {}
            <div>
        </div>
    ", markdown::to_html(&post_.content));

    Ok(content)
}

/// Get all records in the `post` table
/// **Use with caution, since there's no pagination implemented yet**
/// TODO: Finish this documentation (I hate pagination)
pub fn all() -> Vec<Post> {
    let conn = &mut db::connect_db();
    post::table.order(post::created_at.desc()).load(conn).expect("Error loading posts")
}

/// Find one `Post` in the db
///
/// # Examples
/// ```rust
/// if let Ok(post) = posts::one(my_id) {
///  // do something...
/// }
/// ```
pub fn one(id: i32) -> QueryResult<Post> {
    let conn = &mut db::connect_db();
    post::table.find(id).first(conn)
}

