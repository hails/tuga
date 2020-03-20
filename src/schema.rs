table! {
    processes (id) {
        id -> Int4,
        code -> Text,
        telegram_user_id -> Text,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
