table! {
    links (short) {
        short -> Text,
        long -> Text,
        notes -> Text,
        created_at -> Timestamptz,
        created_by -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        pw_hash -> Text,
        orig -> Bool,
        manage_links -> Bool,
        manage_users -> Bool,
        disabled -> Bool,
    }
}

joinable!(links -> users (created_by));

allow_tables_to_appear_in_same_query!(links, users,);
