use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::post)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug)]
// Peculiar thing about this, the order **must** match the order in the table
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub slug: String,
    pub tags: String,
    // Since `abstract is a reserved keyword we're using `description` in rust
    pub description: String,
}
