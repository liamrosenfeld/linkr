table! {
    links (id) {
        id -> Int4,
        short -> Text,
        long -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        pw_hash -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    links,
    users,
);
