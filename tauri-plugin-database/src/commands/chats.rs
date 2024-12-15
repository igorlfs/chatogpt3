use crate::{
    connection::establish_connection,
    models::Chat,
    objects::chats::{add, delete, list, update},
};

#[tauri::command]
pub fn add_chat(title: &str, summary: &str) -> i32 {
    let conn = &mut establish_connection();

    let todo = add(conn, title, summary);

    todo.id
}

#[tauri::command]
pub fn list_chats() -> Vec<Chat> {
    let conn = &mut establish_connection();

    list(conn)
}

#[tauri::command]
pub fn delete_chat(id: i32) {
    let conn = &mut establish_connection();

    delete(conn, id)
}

#[tauri::command]
pub fn update_chat(id: i32, title: &str, summary: &str) -> Chat {
    let conn = &mut establish_connection();

    update(conn, id, title, summary)
}
