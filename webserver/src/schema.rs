// @generated automatically by Diesel CLI.

diesel::table! {
    message (id) {
        id -> Int4,
        content -> Text,
        user_id -> Text,
        time_sent -> Timestamp,
        room_id -> Int4,
    }
}

diesel::table! {
    room (id) {
        id -> Int4,
        title -> Text,
        owner_id -> Text,
    }
}

diesel::joinable!(message -> room (room_id));

diesel::allow_tables_to_appear_in_same_query!(
    message,
    room,
);
