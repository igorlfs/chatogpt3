pub mod cipher;
mod commands;
pub mod gemini;
pub mod strings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_database::init())
        .invoke_handler(tauri::generate_handler![commands::get_bot_reply])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
