use rand::{thread_rng, Rng};
use std::env;
use tauri_plugin_database::commands::messages::get_messages;

use crate::{
    cipher::{caesar_cipher, vigenere_cipher},
    gemini::{
        get_chat_response,
        lib::{Content, Part},
    },
    strings::{
        alternate_string_case, is_string_a_password_secure, is_string_an_email_address,
        is_string_ordered,
    },
};

#[tauri::command]
pub async fn get_bot_reply(message: String, chat_id: i32) -> String {
    let apikey = env::var("APIKEY");

    // TODO we shouldn't always default to gemini
    if let Ok(apikey) = apikey {
        let chat_content = get_messages(chat_id)
            .into_iter()
            .map(|message| Content {
                role: if message.author == "user" {
                    String::from("user")
                } else {
                    String::from("model")
                },
                parts: vec![Part::Text(message.content)],
            })
            .collect();

        let (reply, _) = get_chat_response(&apikey, chat_content).await;

        match reply {
            Some(reply) => return reply,
            None => return String::from("An error occured"),
        };
    }

    const NUM_POSSIBLE_ANSWERS: i32 = 7;

    let message = message.as_str();

    match thread_rng().gen_range(0..=NUM_POSSIBLE_ANSWERS) {
        0 => String::from("Pong!"),
        // String-related methods
        1 => is_string_an_email_address(message),
        2 => is_string_ordered(message),
        3 => is_string_a_password_secure(message),
        4 => alternate_string_case(message),
        // Cipher-related methods
        5 => caesar_cipher(message, 13),        // ROT 13
        _ => vigenere_cipher(message, "syrax"), // Fire && Blood
    }
}
