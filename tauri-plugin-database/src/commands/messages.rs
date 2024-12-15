use crate::{
    connection::establish_connection,
    models::{Message, NewMessage},
};
use diesel::prelude::*;

#[tauri::command]
pub fn add_message(new_message: NewMessage) -> i32 {
    let conn = &mut establish_connection();

    use crate::schema::messages;

    let todo = diesel::insert_into(messages::table)
        .values(&new_message)
        .returning(Message::as_returning())
        .get_result(conn)
        .expect("Error adding new message");

    todo.id
}

#[tauri::command]
pub fn get_messages(chat: i32) -> Vec<Message> {
    let conn = &mut establish_connection();

    use crate::schema::messages::dsl::*;

    messages
        .filter(chat_id.is(chat))
        .select(Message::as_select())
        .load(conn)
        .expect("Error loading chats")
}
