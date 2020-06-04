table! {
    links (id) {
        id -> Int4,
        short -> Varchar,
        long -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    links,
    users,
);
