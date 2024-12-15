diesel::table! {
    chats (id) {
        id -> Integer,
        title -> Text,
        summary -> Text,
    }
}

diesel::table! {
    messages (id) {
        id -> Integer,
        content -> Text,
        author -> Text,
        chat_id -> Integer,
    }
}
