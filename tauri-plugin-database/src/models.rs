use super::schema::chats;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, PartialEq)]
#[diesel(table_name = chats)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Chat {
    pub id: i32,
    pub title: String,
    pub summary: String,
}

#[derive(Insertable)]
#[diesel(table_name = chats)]
pub struct NewChat<'a> {
    pub title: &'a str,
    pub summary: &'a str,
}

use super::schema::messages;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize)]
#[diesel(belongs_to(Chat))]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Message {
    pub id: i32,
    pub content: String,
    pub author: String,
    pub chat_id: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = messages)]
#[serde(rename_all = "camelCase")]
pub struct NewMessage<'a> {
    pub content: &'a str,
    pub author: &'a str,
    pub chat_id: i32,
}
