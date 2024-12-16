use rand::{thread_rng, Rng};

use crate::strings::{
    alternate_string_case, is_string_a_password_secure, is_string_an_email_address,
    is_string_ordered,
};

#[tauri::command]
pub fn get_bot_reply(message: &str) -> String {

    const NUM_POSSIBLE_ANSWERS: i32 = 5;

    match thread_rng().gen_range(0..=NUM_POSSIBLE_ANSWERS) {
        0 => "Pong!".to_string(),
        1 => is_string_an_email_address(message),
        2 => is_string_ordered(message),
        3 => is_string_a_password_secure(message),
        _ => alternate_string_case(message),
    }
}
