use crate::models::{Chat, NewChat};
use diesel::prelude::*;

pub fn add(conn: &mut SqliteConnection, new_chat: NewChat) -> i32 {
    use crate::schema::chats;

    diesel::insert_into(chats::table)
        .values(&new_chat)
        .returning(Chat::as_returning())
        .get_result(conn)
        .expect("Error adding new chat")
        .id
}

pub fn list(conn: &mut SqliteConnection) -> Vec<Chat> {
    use crate::schema::chats::dsl::*;

    chats
        .select(Chat::as_select())
        .load(conn)
        .expect("Error loading chats")
}

pub fn delete(conn: &mut SqliteConnection, chat_id: i32) {
    use crate::schema::chats::dsl::*;

    diesel::delete(chats.filter(id.eq(chat_id)))
        .execute(conn)
        .unwrap_or_else(|_| panic!("Error deleting chat with id `{chat_id}`"));
}

pub fn update(conn: &mut SqliteConnection, chat: Chat) -> Chat {
    use crate::schema::chats::dsl::*;

    diesel::update(chats.find(chat.id))
        .set((title.eq(chat.title), summary.eq(chat.summary)))
        .returning(Chat::as_returning())
        .get_result(conn)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::{add, delete, list, update};
    use crate::models::{Chat, NewChat};
    use diesel::{Connection, SqliteConnection};
    use diesel_migrations::MigrationHarness;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

    #[test]
    fn add_chat() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        let id = add(
            &mut conn,
            NewChat {
                title: String::from("foo"),
                summary: String::from("bar"),
            },
        );

        assert!(id == 1)
    }

    #[test]
    fn list_no_chats() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        let chat_list = list(&mut conn);

        assert!(chat_list == vec![]);
    }

    #[test]
    fn list_single_chat() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        let title = "foo";
        let summary = "bar";

        let new_chat = NewChat {
            title: String::from(title),
            summary: String::from(summary),
        };

        add(&mut conn, new_chat);

        let chat_list = list(&mut conn);

        let expected_chat = Chat {
            title: String::from(title),
            summary: String::from(summary),
            id: 1,
        };

        assert!(chat_list == vec![expected_chat]);
    }

    #[test]
    fn delete_chat() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        let new_chat = NewChat {
            title: String::from("foo"),
            summary: String::from("bar"),
        };

        add(&mut conn, new_chat);

        delete(&mut conn, 1);

        let chat_list = list(&mut conn);

        assert!(chat_list == vec![]);
    }

    #[test]
    fn update_title_and_summary() {
        let mut conn = SqliteConnection::establish(":memory:").unwrap();

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        let new_chat = NewChat {
            title: String::from("foo"),
            summary: String::from("bar"),
        };

        add(&mut conn, new_chat);

        let chat_update = Chat {
            title: String::from("fizz"),
            summary: String::from("buzz"),
            id: 1,
        };

        let updated_chat = update(&mut conn, chat_update);

        assert!(updated_chat.title == "fizz");
        assert!(updated_chat.summary == "buzz");
    }
}
