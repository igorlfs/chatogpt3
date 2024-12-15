use crate::models::{Chat, NewChat};
use diesel::prelude::*;

pub fn add(conn: &mut SqliteConnection, title: &str, summary: &str) -> Chat {
    use crate::schema::chats;

    let new_chat = NewChat { title, summary };

    diesel::insert_into(chats::table)
        .values(&new_chat)
        .returning(Chat::as_returning())
        .get_result(conn)
        .expect("Error adding new chat")
}

pub fn list(conn: &mut SqliteConnection) -> Vec<Chat> {
    use crate::schema::chats::dsl::*;

    chats
        .select(Chat::as_select())
        .load(conn)
        .expect("Error loading chats")
}

pub fn delete(conn: &mut SqliteConnection, id_: i32) {
    use crate::schema::chats::dsl::*;

    diesel::delete(chats.filter(id.eq(id_)))
        .execute(conn)
        .expect("Error deleting posts");
}

pub fn update(conn: &mut SqliteConnection, id_: i32, title_: &str, summary_: &str) -> Chat {
    use crate::schema::chats::dsl::*;

    diesel::update(chats.find(id_))
        .set((title.eq(title_), summary.eq(summary_)))
        .returning(Chat::as_returning())
        .get_result(conn)
        .unwrap()
}
