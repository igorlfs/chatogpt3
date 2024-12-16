-- https://stackoverflow.com/a/5901100
PRAGMA foreign_keys = ON;

CREATE TABLE messages (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    author VARCHAR CHECK(author IN ('bot', 'user')) NOT NULL,
    chat_id INTEGER NOT NULL,
    FOREIGN KEY(chat_id) REFERENCES chats(id) ON DELETE CASCADE
)
