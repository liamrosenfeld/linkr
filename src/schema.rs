table! {
    links (short) {
        short -> Text,
        long -> Text,
        created_at -> Timestamptz,
        created_by -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        pw_hash -> Text,
        manage_links -> Bool,
        manage_users -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(links, users,);
