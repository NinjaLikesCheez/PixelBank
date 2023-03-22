// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        created_at -> Text,
        balance -> Integer,
        role -> Integer,
    }
}
