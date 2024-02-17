// @generated automatically by Diesel CLI.

diesel::table! {
    post (id) {
        id -> Integer,
        title -> Text,
        content -> Text,
        created_at -> Text,
    }
}
