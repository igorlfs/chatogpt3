<script lang="ts">
  import BotIcon from "$lib/components/icons/BotIcon.svelte";
  import SendIcon from "$lib/components/icons/SendIcon.svelte";
  import UserIcon from "$lib/components/icons/UserIcon.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { Chat, Message, NewChat, NewMessage } from "./types";
  import type { PageData } from "./$types";
  import PlusIcon from "$lib/components/icons/PlusIcon.svelte";
  import TrashIcon from "$lib/components/icons/TrashIcon.svelte";
  import EditIcon from "$lib/components/icons/EditIcon.svelte";
  import SaveIcon from "$lib/components/icons/SaveIcon.svelte";
  import { onMount } from "svelte";

  let currentMessage = $state("");

  const { data }: { data: PageData } = $props();

  const chats: Chat[] = $state(data.chats);

  let selectedChat = $state(0);
  let history: Message[] = $state([]);

  onMount(async () => {
    await updateHistory();
  });

  const updateHistory = async () => {
    history = await invoke("plugin:database|get_messages", {
      chat: chats[selectedChat].id,
    });
  };

  const sendMessage = async () => {
    const chatId = chats[selectedChat].id;

    const userMessage: NewMessage = {
      content: currentMessage,
      author: "user",
      chatId,
    };

    const userMessageId: number = await invoke("plugin:database|add_message", {
      newMessage: { ...userMessage },
    });

    history.push({ ...userMessage, id: userMessageId });

    const reply: string = await invoke("get_bot_reply", {
      message: currentMessage,
      chatId,
    });

    const botMessage: NewMessage = {
      content: reply,
      author: "bot",
      chatId,
    };

    const botMessageId: number = await invoke("plugin:database|add_message", {
      newMessage: { ...botMessage },
    });

    history.push({ ...botMessage, id: botMessageId });
  };

  const addChat = async () => {
    const i = chats.length;

    const newChat: NewChat = {
      title: "New Chat",
      summary: `Chat #${i + 1}`,
    };

    const id: number = await invoke("plugin:database|add_chat", {
      newChat: { ...newChat },
    });

    chats.push({ ...newChat, id });

    // If there were no chats and we just added in the first one,
    // We have to update `selectedChat` so it doesn't contain garbage
    if (chats.length === 1) {
      selectedChat = 0;
    }
  };

  const deleteChat = async (idx: number) => {
    await invoke("plugin:database|delete_chat", { chatId: chats[idx].id });

    chats.splice(idx, 1);

    if (idx === selectedChat) {
      // If we removed the last chat, its index no longer belongs in the array
      selectedChat = idx === chats.length ? idx - 1 : idx;
      // But we also have to handle the case where there's a single chat
      if (selectedChat >= 0) {
        await updateHistory();
      } else {
        history = [];
      }
    }
  };

  const updateChat = async (idx: number) => {
    chats[idx] = await invoke("plugin:database|update_chat", {
      chat: { ...editingChat, id: chats[idx].id },
    });

    editingChatIndex = -1;
  };

  let editingChatIndex = $state(-1);
  let editingChat: NewChat = $state({ title: "", summary: "" });

  $effect(() => {
    if (editingChatIndex !== -1) {
      editingChat = chats[editingChatIndex];
    }
  });
</script>

<div class="flex">
  <div class="basis-1/4 h-screen flex flex-col bg-mantle overflow-y-auto">
    <div class="flex flex-col gap-3">
      <ul>
        {#each chats as chat, i}
          {@const isSelected = selectedChat === i}
          <li
            class={`${isSelected ? "bg-surface0" : "bg-crust hover:bg-base"} px-3 py-2 border-b-base border-b-2`}
          >
            <div class="flex justify-between">
              {#if editingChatIndex === i}
                <div>
                  <input
                    bind:value={editingChat.title}
                    class="m-2 bg-crust text-text p-1 rounded-md w-32"
                  />
                  <input
                    bind:value={editingChat.summary}
                    class="m-2 bg-crust text-text p-1 rounded-md w-32"
                  />
                </div>
              {:else}
                {@const title = chat.title}
                {@const chatTitle =
                  title.length > 9 ? `${title.slice(0, 9)}...` : title}
                {@const summary = chat.summary}
                {@const chatSummary =
                  summary.length > 10 ? `${summary.slice(0, 10)}...` : summary}
                <div
                  class="cursor-default flex-1"
                  aria-hidden={true}
                  onclick={async () => {
                    selectedChat = i;
                    await updateHistory();
                  }}
                >
                  <p class="text-text">{chatTitle}</p>
                  <p class="text-subtext text-sm">{chatSummary}</p>
                </div>
              {/if}
              <div class="flex flex-row gap-3">
                {#if editingChatIndex !== i}
                  <button
                    class={`${isSelected ? "text-overlay0" : "text-surface1"} hover:text-lavender rounded-full`}
                    onclick={() => (editingChatIndex = i)}
                  >
                    <EditIcon />
                  </button>
                  <button
                    class={`${isSelected ? "text-overlay0" : "text-surface1"} hover:text-lavender rounded-full`}
                    onclick={async () => await deleteChat(i)}
                  >
                    <TrashIcon />
                  </button>
                {:else}
                  <button
                    class={`${isSelected ? "text-overlay0" : "text-surface1"} hover:text-lavender rounded-full`}
                    onclick={async () => await updateChat(i)}
                  >
                    <SaveIcon />
                  </button>
                {/if}
              </div>
            </div>
          </li>
        {/each}
      </ul>
      <button
        class="bg-primary text-crust p-1 rounded-full self-center mb-10"
        onclick={addChat}
      >
        <PlusIcon />
      </button>
    </div>
  </div>

  <div class="basis-3/4 flex flex-col justify-between">
    <div class="overflow-y-auto flex flex-col h-[85vh]">
      {#each history as message}
        {@const isUser = message.author === "user"}
        <div
          class={`flex ${isUser ? "flex-row" : "flex-row-reverse"} items-center *:my-2`}
        >
          <div class="mx-2 text-text">
            {#if isUser}
              <UserIcon />
            {:else}
              <BotIcon />
            {/if}
          </div>
          <p
            class={`text-text bg-mantle rounded-lg p-2 ${isUser ? "mr-2" : "ml-2"}`}
          >
            {message.content}
          </p>
        </div>
      {/each}
    </div>

    <form onsubmit={sendMessage} class="flex flex-row items-center h-[10vh]">
      <textarea
        class="flex-1 mx-2 bg-crust mb-3 text-text py-1 px-2 rounded-md z-100"
        placeholder="Write a message"
        bind:value={currentMessage}
        onkeypress={async (event) => {
          if (event.key === "Enter" && !event.shiftKey) {
            // Don't output newline
            event.preventDefault();

            await sendMessage();

            // Reset message
            currentMessage = "";
          }
        }}
      ></textarea>
      <button
        class="bg-primary text-crust p-2 rounded-xl mr-2 mb-3"
        type="submit"
        aria-label="Send"
      >
        <SendIcon />
      </button>
    </form>
  </div>
</div>
