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
                <button onclick={startCreatingNewWorld}>
                    create new world
                </button>
            {:else}
                <input bind:value={newWorldName} onkeydown={handleKeydown} />
            {/if}
        </div>
        <WorldList />
    </div>
</main>
