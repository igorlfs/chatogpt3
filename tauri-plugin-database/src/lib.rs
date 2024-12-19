pub mod commands;
mod connection;
pub mod db;
mod models;
mod schema;

use commands::chats::*;
use commands::messages::*;
use connection::{establish_connection, get_connection_url};
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};
use std::error::Error;
use std::fs::create_dir_all;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

pub fn setup_db(connection_url: &str) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    {
        // Create a new scope so we can release the lock before establishing the connection
        let mut locked_url = get_connection_url().lock().unwrap();
        *locked_url = connection_url.to_owned();
    }

    let mut conn = establish_connection();

    conn.run_pending_migrations(MIGRATIONS)
        .unwrap_or_else(|error| panic!("Error running database migrations: {error:?}"));

    Ok(())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("database")
        .invoke_handler(tauri::generate_handler![
            add_chat,
            list_chats,
            delete_chat,
            update_chat,
            add_message,
            get_messages,
        ])
        .setup(|app, _| {
            let app_path = app
                .path()
                .app_local_data_dir()
                .expect("No App path was found!");

            // See https://github.com/bpevs/tauri_diesel_sqlite_example/blob/main/src-db/src/lib.rs
            // This man is a literal god send

            let db_file_name = "database.db";

            let conn_url = format!("sqlite://{}/{}", app_path.display(), db_file_name);

            if let Err(e) = create_dir_all(&app_path) {
                println!("Problem creating app directory: {:?}", e);
            }

            if let Err(e) = setup_db(&conn_url) {
                println!("Database setup failed: {:?}", e);
            }

            Ok(())
        })
        .build()
}
