export type Chat = { title: string; summary: string; id: number };

export type NewChat = Omit<Chat, "id">;

export type Message = {
  content: string;
  author: "user" | "bot";
  chatId: number;
  id: number;
};

export type NewMessage = Omit<Message, "id">;
