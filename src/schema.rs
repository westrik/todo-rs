table! {
    tasks (id) {
        id -> Int4,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        resolved_at -> Nullable<Timestamptz>,
    }
}
