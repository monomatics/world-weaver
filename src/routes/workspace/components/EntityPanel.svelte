<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { worldState, entityState } from "$lib/workspaceStates.svelte";
    import { type Entity } from "$lib/dashboardUniversals.svelte";

    let isCreatingNewEntity = $state(false);
    let isLoadingEntities = $state(true);
    let entityName = $state("untitled");
    let entityList = $state<Entity[]>([]);

    function startCreatingNewEntity() {
        isCreatingNewEntity = true;
    }

    function stopCreatingNewEntity() {
        isCreatingNewEntity = false;
    }

    async function createNewEntity() {
        const name = entityName.trim();
        const category = entityState.savedEntityCategoryName?.trim();
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
        if (e.key === "Enter") createNewEntity();
        if (e.key === "Escape") stopCreatingNewEntity();
    }

    async function listEntitiesInCategory(category: string) {
        isLoadingEntities = true;
        try {
            const entities: Entity[] = await invoke(
                "list_entities_in_category",
                {
                    worldId: worldState.savedWorldId,
                    category: category,
                },
            );
            entityList = entities;
        } catch (e) {
            console.error(
                `An error occurred while fetching entities in category ${category}:`,
                e,
            );
        } finally {
            isLoadingEntities = false;
        }
    }

    $effect(() => {
        if (worldState.savedWorldId) {
            listEntitiesInCategory(entityState.savedEntityCategoryName?);
        }
    });
</script>

<main class="entity-panel-container">
    {#if isLoadingEntities}
        loading entities...
    {:else if entityList.length === 0}
        no entities found.
    {:else}
        {#each entityList as entity (entity.id)}
            <div>
                <button>{entity.name}</button>
            </div>
        {/each}
    {/if}

    {#if !isCreatingNewEntity}
        <div>
            <button onclick={startCreatingNewEntity}>create new entity</button>
        </div>
    {:else}
        <div><input bind:value={entityName} onkeydown={handleKeydown} /></div>
        <div>
            <input bind:value={entityState.savedEntityCategoryName} onkeydown={handleKeydown} />
        </div>
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
