use rand::{thread_rng, Rng};

use crate::strings::{alternate_string_case, is_string_ordered, match_email_address};

#[tauri::command]
pub fn get_bot_reply(message: &str) -> String {
    const NUM_POSSIBLE_ANSWERS: i32 = 4;
    let reply_type = thread_rng().gen_range(0..=NUM_POSSIBLE_ANSWERS);


    match reply_type {
        0 => match_email_address(message),
        1 => is_string_ordered(message),
        2 => alternate_string_case(message),
        _ => "Pong!".to_string(),
    }
}
