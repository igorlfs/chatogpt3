use super::schema::chats;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
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
