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

diesel::table! {
    transactions (id) {
        id -> Integer,
        account -> Text,
        created_at -> Text,
        kind -> Text,
        mutation -> Integer,
        recipient -> Text,
    }
}
