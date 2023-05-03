// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Text,
        name -> Text,
        has_deposit -> Bool,
        price -> Integer,
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
    products,
    users,
);
