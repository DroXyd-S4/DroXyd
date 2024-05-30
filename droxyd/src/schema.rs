// @generated automatically by Diesel CLI.

diesel::table! {
    posts1 (id) {
        id -> Integer,
        url -> Text,
        langue -> Text,
        name -> Text,
        date -> Text,
        word1 -> Text,
        word2 -> Text,
        word3 -> Text,
    }
}

diesel::table! {
    posts2 (i) {
        i -> Integer,
        key -> Text,
        idofsite -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts1,
    posts2,
);
