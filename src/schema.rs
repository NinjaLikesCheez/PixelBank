// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Integer,
        user_id -> Text,
        created_at -> Text,
        kind -> Text,
        mutation -> Integer,
        recipient_id -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        created_at -> Text,
        username -> Text,
        balance -> Integer,
        role -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    transactions,
    users,
);
