// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        created_at -> Text,
        username -> Text,
        balance -> Integer,
        role -> Text,
    }
}
