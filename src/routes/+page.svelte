<script lang="ts">
  import BotIcon from "$lib/components/icons/BotIcon.svelte";
  import SendIcon from "$lib/components/icons/SendIcon.svelte";
  import UserIcon from "$lib/components/icons/UserIcon.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { Chat, NewChat } from "./types";
  import type { PageData } from "./$types";
  import PlusIcon from "$lib/components/icons/PlusIcon.svelte";
  import TrashIcon from "$lib/components/icons/TrashIcon.svelte";
  import EditIcon from "$lib/components/icons/EditIcon.svelte";
  import SaveIcon from "$lib/components/icons/SaveIcon.svelte";

  let history: string[] = $state([]);
  let currentMessage = $state("");

  let selectedConversation = $state(0);

  let { data }: { data: PageData } = $props();

  let chats: Chat[] = $state(data.chats);

  const sendMessage = async () => {
    history.push(currentMessage);
  };

  const addChat = async () => {
    const i = chats.length;

    const newChat: NewChat = {
      title: "New Chat",
      summary: `Chat #${i + 1}`,
    };

    const id: number = await invoke("plugin:database|add_chat", newChat);

    chats.push({ ...newChat, id });
  };

  const deleteChat = async (idx: number) => {
    const id = chats[idx].id;

    await invoke("plugin:database|delete_chat", { id });

    chats.splice(idx, 1);
  };

  const updateChat = async (idx: number) => {
    chats[idx] = await invoke("plugin:database|update_chat", {
      id: chats[idx].id,
      ...editingChat,
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

  $inspect(chats);
</script>

<div class="flex">
  <div class="basis-1/4 h-screen flex flex-col bg-crust overflow-y-auto">
    <div class="flex flex-col gap-3">
      <ul>
        {#each chats as chat, i}
          <li
            class={`${selectedConversation === i ? "bg-base" : "bg-mantle"} px-3 hover:bg-base py-2 border-b-base border-b-2`}
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
                  title.length > 12 ? `${title.slice(0, 12)}...` : title}
                {@const summary = chat.summary}
                {@const chatSummary =
                  summary.length > 14 ? `${summary.slice(0, 14)}...` : summary}
                <div>
                  <p class="text-text">{chatTitle}</p>
                  <p class="text-subtext text-sm">{chatSummary}</p>
                </div>
              {/if}
              <div class="flex flex-row gap-3">
                {#if editingChatIndex !== i}
                  <button
                    class="text-surface2 hover:text-lavender rounded-full"
                    onclick={() => (editingChatIndex = i)}
                  >
                    <EditIcon />
                  </button>
                  <button
                    class="text-surface2 hover:text-lavender rounded-full"
                    onclick={async () => await deleteChat(i)}
                  >
                    <TrashIcon />
                  </button>
                {:else}
                  <button
                    class="text-surface2 hover:text-lavender rounded-full"
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
      {#each history as message, i}
        {@const isEven = i % 2 === 0}
        <div
          class={`flex ${isEven ? "flex-row" : "flex-row-reverse"} items-center *:my-2`}
        >
          <div class="mx-2 text-text">
            {#if isEven}
              <UserIcon />
            {:else}
              <BotIcon />
            {/if}
          </div>
          <p
            class={`text-text bg-mantle rounded-lg p-2 ${isEven ? "mr-2" : "ml-2"}`}
          >
            {message}
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
