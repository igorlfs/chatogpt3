use crate::{
    connection::establish_connection,
    db::chats,
    models::{Chat, NewChat},
};

#[tauri::command]
pub async fn add_chat(new_chat: NewChat) -> i32 {
    let conn = &mut establish_connection();

    chats::add(conn, new_chat)
}

#[tauri::command]
pub async fn list_chats() -> Vec<Chat> {
    let conn = &mut establish_connection();

    chats::list(conn)
}

#[tauri::command]
pub async fn delete_chat(chat_id: i32) {
    let conn = &mut establish_connection();

    chats::delete(conn, chat_id)
}

#[tauri::command]
pub async fn update_chat(chat: Chat) -> Chat {
    let conn = &mut establish_connection();

    chats::update(conn, chat)
}
