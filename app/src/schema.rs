table! {
    posts (id) {
        id -> Int4,
        title -> Text,
        body -> Text,
        is_published -> Bool,
        create_time -> Timestamptz,
        update_time -> Timestamptz,
    }
}
