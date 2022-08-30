// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        author_id -> Int4,
    }
}

diesel::joinable!(posts -> authors (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    posts,
);
