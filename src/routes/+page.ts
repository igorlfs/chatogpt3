import type { Load } from "@sveltejs/kit";
import { invoke } from "@tauri-apps/api/core";
import type { Chat } from "./types";

export const load: Load = async () => {
  const chats: Chat[] = await invoke("plugin:database|list_chats");

  if (chats.length === 0) {
    const newChat = {
      title: "New Chat",
      summary: "Your first chat",
    };

    const id: number = await invoke("plugin:database|add_chat", {
      title: "New Chat",
      summary: "Your first chat",
    });

    chats.push({ ...newChat, id });
  }

  return { chats };
};
