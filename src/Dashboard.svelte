<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import WorldList from "./components/WorldList.svelte";

    let isCreatingNewWorld = $state(false);
    let newWorldName = $state("untitled");

    function startCreatingNewWorld() {
        isCreatingNewWorld = true;
    }

    function stopCreatingNewWorld() {
        isCreatingNewWorld = false;
    }

    async function createNewWorld() {
        await invoke("create_new_world", { name: newWorldName });
        stopCreatingNewWorld();
    }
    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Escape") stopCreatingNewWorld();
        if (e.key === "Enter") createNewWorld();
    }
</script>

<main>
    <div style="display:flex;">
        <div>
            {#if !isCreatingNewWorld}
                <button
                    class="dashboard-button"
                    onclick={startCreatingNewWorld}
                >
                    create new world
                </button>
            {:else}
                <input bind:value={newWorldName} onkeydown={handleKeydown} />
            {/if}
        </div>
        <WorldList />
    </div>
</main>

<style>
    .dashboard-button {
        border: none;
        color: var(--text);
        background-color: var(--overlay);
        font-size: 1rem;
        padding: 0.5rem;
        margin: 0.2rem;
        border-radius: 0.5rem;
    }
    .dashboard-button:hover{
        background-color: var(--surface);
    }
</style>
