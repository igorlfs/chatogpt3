use std::env;
use tauri_plugin_database::commands::messages::get_messages;

use crate::gemini::{
    get_chat_response,
    lib::{Content, Part},
};

#[tauri::command]
pub async fn get_bot_reply(message: String, chat_id: i32) -> String {
    let apikey = env::var("APIKEY");

    if let Ok(apikey) = apikey {
        let chat_content = get_messages(chat_id)
            .await
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

    message
}
