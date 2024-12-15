use diesel::prelude::*;
use std::sync::Mutex;

use std::sync::OnceLock;

pub fn get_connection_url() -> &'static Mutex<String> {
    static CONNECTION_URL: OnceLock<Mutex<String>> = OnceLock::new();
    CONNECTION_URL.get_or_init(|| Mutex::new(String::new()))
}

pub fn establish_connection() -> SqliteConnection {
    let connection_url = get_connection_url().lock().unwrap();

    SqliteConnection::establish(&connection_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", *connection_url))
}
