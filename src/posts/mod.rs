//! Contains abstracions related to Posts; mainly db queries
use super::db;
use super::db::models::*;
use super::db::schema::*;
use diesel::prelude::*;
//TODO: test these methods

/// Get all records in the `post` table showing the most recent first
///
/// **Use with caution, since there's no pagination implemented yet**
///
/// TODO: Finish this documentation (I hate pagination)
pub fn all() -> Vec<Post> {
    let conn = &mut db::connect_db();
    post::table.order(post::created_at.desc()).load(conn).expect("Error loading posts")
}

/// Find one `Post` in the db
///
/// # Examples
/// ```rust
///
/// let my_id = 1;
/// if let Ok(post) = posts::one(my_id) {
///  // do something...
/// }
/// ```
pub fn one(id: i32) -> QueryResult<Post> {
    let conn = &mut db::connect_db();
    post::table.find(id).first(conn)
}

