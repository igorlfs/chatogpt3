<script lang="ts">
    import "../app.css";
    import type { Snippet } from "svelte";

    let { children }: { children: Snippet } = $props();
</script>

<div class="flex flex-col min-h-screen justify-between">
    <div>
        <!-- <Header /> -->

        <main>
            {@render children()}
        </main>
    </div>
    <!-- See https://stackoverflow.com/a/67846892 -->
    <!-- <Footer /> -->
</div>
