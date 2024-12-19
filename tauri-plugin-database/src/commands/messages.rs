use crate::{
    connection::establish_connection,
    db::messages,
    models::{Message, NewMessage},
};

#[tauri::command]
pub async fn add_message(new_message: NewMessage) -> i32 {
    let conn = &mut establish_connection();

    messages::add(conn, new_message)
}

#[tauri::command]
pub async fn get_messages(chat: i32) -> Vec<Message> {
    let conn = &mut establish_connection();

    messages::list(conn, chat)
}
