<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { worldState } from "$lib/workspaceStates.svelte";
    let isCreatingNewEntity = $state(false);
    let entityName = $state("untitled");
    let entityCategory = $state("default");

    function startCreatingNewEntity() {
        isCreatingNewEntity = true;
    }

    function stopCreatingNewEntity() {
        isCreatingNewEntity = false;
    }

    async function createNewEntity() {
        const name = entityName.trim();
        const category = entityCategory.trim();
        if (!name || !category) return;
        try {
            await invoke("create_new_entity", {
                entityName: name,
                entityCategory: category,
                worldId: worldState.savedWorldId,
            });
            stopCreatingNewEntity();
        } catch (e) {
            console.error("an error occured:", e);
        }
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === 'Enter') createNewEntity();
        if (e.key === 'Escape') stopCreatingNewEntity();
    }
</script>

<main class="entity-panel-container">
    {#if !isCreatingNewEntity}
        <div>
            <button onclick={startCreatingNewEntity}>create new entity</button>
        </div>
    {:else}
        <div><input bind:value={entityName} onkeydown={handleKeydown} /></div>
        <div><input bind:value={entityCategory} onkeydown={handleKeydown}/></div>
        <div><button>create new Entity</button></div>
    {/if}
</main>

<style>
    .entity-panel-container {
        background-color: var(--surface);
        padding: 0.5rem;
        margin: 0.5rem;
    }
</style>
