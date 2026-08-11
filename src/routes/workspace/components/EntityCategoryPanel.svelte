<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import {
        worldState,
        entityCategoryState,
        entityState
    } from "$lib/workspaceStates.svelte";

    let entityCategories = $state<string[]>([]);

    async function fetchEntityCategories() {
        try {
            const categories: string[] = await invoke(
                "list_entity_categories",
                {
                    worldId: worldState.savedWorldId,
                },
            );
            entityCategories = categories;
        } catch (e) {
            console.error(
                "An error occurred while fetching entity categories:",
                e,
            );
        }
    }

    $effect(() => {
        if (worldState.savedWorldId) {
            fetchEntityCategories();
        }
    });
</script>

<main>
    <b>Entity Categories</b>
    {#each entityCategories as category}
        <div>
            <button
                onclick={() =>
                    entityCategoryState.setEntityCategoryState(category)}
                >{category}</button
            >
        </div>
    {/each}
</main>
