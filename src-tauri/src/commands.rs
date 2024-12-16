use rand::{thread_rng, Rng};

use crate::{
    cipher::{caesar_cipher, vigenere_cipher},
    strings::{
        alternate_string_case, is_string_a_password_secure, is_string_an_email_address,
        is_string_ordered,
    },
};

#[tauri::command]
pub fn get_bot_reply(message: &str) -> String {

    const NUM_POSSIBLE_ANSWERS: i32 = 7;

    match thread_rng().gen_range(0..=NUM_POSSIBLE_ANSWERS) {
        0 => "Pong!".to_string(),
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
