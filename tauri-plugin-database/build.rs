const COMMANDS: &[&str] = &[
    "add_chat",
    "list_chats",
    "delete_chat",
    "update_chat",
    "add_message",
    "get_messages",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
