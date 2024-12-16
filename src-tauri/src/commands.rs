use rand::{thread_rng, Rng};

#[tauri::command]
pub fn get_bot_reply(message: &str) -> &str {
    let reply_type = thread_rng().gen_range(0..=1);

    match reply_type {
        _ => "Pong!",
    }
}
