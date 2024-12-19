use crate::models::{Message, NewMessage};
use diesel::prelude::*;

pub fn add(conn: &mut SqliteConnection, new_message: NewMessage) -> i32 {
    use crate::schema::messages;

    diesel::insert_into(messages::table)
        .values(&new_message)
        .returning(Message::as_returning())
        .get_result(conn)
        .expect("Error adding new message")
        .id
}

pub fn list(conn: &mut SqliteConnection, chat: i32) -> Vec<Message> {
    use crate::schema::messages::dsl::*;

    messages
        .filter(chat_id.is(chat))
        .select(Message::as_select())
        .load(conn)
        .expect("Error getting messages")
}

#[cfg(test)]
mod test {
    use super::{add, list};
    use crate::models::{Message, NewMessage};
    use diesel::{Connection, SqliteConnection};
    use diesel_migrations::MigrationHarness;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

    #[test]
    fn add_message() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        let id = add(
            &mut conn,
            NewMessage {
                content: String::from("foo"),
                author: String::from("user"),
                chat_id: 1,
            },
        );

        assert!(id == 1)
    }

    #[test]
    #[should_panic(expected = "Error adding new message")]
    fn cant_add_message_with_unknown_author() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        add(
            &mut conn,
            NewMessage {
                content: String::from("foo"),
                author: String::from("bar"),
                chat_id: 1,
            },
        );
    }

    #[test]
    fn list_no_messages() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        let messages_list = list(&mut conn, 1);

        assert!(messages_list == vec![]);
    }

    #[test]
    fn list_single_message() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        let content = "foo";
        let author = "user";

        let new_message = NewMessage {
            content: String::from(content),
            author: String::from(author),
            chat_id: 1,
        };

        add(&mut conn, new_message);

        let message_list = list(&mut conn, 1);

        let expected_message = Message {
            content: String::from(content),
            author: String::from(author),
            chat_id: 1,
            id: 1,
        };

        assert!(message_list == vec![expected_message]);
    }
}
