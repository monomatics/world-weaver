<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { worldState, entityState, entityCategoryState } from "$lib/workspaceStates.svelte";
    import { type Entity } from "$lib/dashboardUniversals.svelte";

    let entity = $state<Entity | null>(null);

    async function fetchEntity(entity_id: string) {
        if (!entity_id) return;
        try {
            const fetchedEntity: Entity = await invoke("get_entity", {
                worldId: worldState.savedWorldId,
                category: entityCategoryState.savedEntityCategoryName,
                entityId: entity_id,
            });
            entity = fetchedEntity;
        } catch (e) {
            console.error("An error occurred while fetching the entity:", e);
        }
    }

    $effect(() => {
        if (!entityState.savedEntityId) return;
        fetchEntity(entityState.savedEntityId);
    });
</script>

<main>
    {#if entity}
        <b>Entity Metadata</b>
        <div><b>Name:</b> {entity.name}</div>
        <div><b>Category:</b> {entity.category}</div>
        <div><b>ID:</b> {entity.id}</div>
    {:else}
        <div>No entity selected.</div>
    {/if}
</main>