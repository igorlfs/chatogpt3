export type Chat = { title: string; summary: string; id: number };

export type NewChat = Omit<Chat, "id">;
