use crate::{
    connection::establish_connection,
    models::{Chat, NewChat},
};
use diesel::prelude::*;

#[tauri::command]
pub fn add_chat(new_chat: NewChat) -> i32 {
    let conn = &mut establish_connection();

    use crate::schema::chats;

    diesel::insert_into(chats::table)
        .values(&new_chat)
        .returning(Chat::as_returning())
        .get_result(conn)
        .expect("Error adding new chat")
        .id
}

#[tauri::command]
pub fn list_chats() -> Vec<Chat> {
    let conn = &mut establish_connection();

    use crate::schema::chats::dsl::*;

    chats
        .select(Chat::as_select())
        .load(conn)
        .expect("Error loading chats")
}

#[tauri::command]
pub fn delete_chat(chat_id: i32) {
    let conn = &mut establish_connection();

    use crate::schema::chats::dsl::*;

    diesel::delete(chats.filter(id.eq(chat_id)))
        .execute(conn)
        .expect("Error deleting posts");
}

#[tauri::command]
pub fn update_chat(chat: Chat) -> Chat {
    let conn = &mut establish_connection();

    use crate::schema::chats::dsl::*;

    diesel::update(chats.find(chat.id))
        .set((title.eq(chat.title), summary.eq(chat.summary)))
        .returning(Chat::as_returning())
        .get_result(conn)
        .unwrap()
}
