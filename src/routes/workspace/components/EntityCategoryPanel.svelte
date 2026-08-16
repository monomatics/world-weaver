<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import {
        worldState,
        entityCategoryState,
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

<main class="main-container">
    <span class="categories-label">entity categories</span>
    {#each entityCategories as category}
        <div>
            <button class="categories-button"
                onclick={() =>
                    entityCategoryState.setEntityCategoryState(category)}
                >{category}</button
            >
        </div>
    {/each}
</main>

<style>
    .main-container{
        background-color:var(--surface);
        margin: 0.5rem;
        border-radius: 2rem;
        height: max(40rem, 85vh);
        width: 15%;
    }
   .categories-label{
       margin: 1rem;
       padding: 1rem;
       text-align: center;
   }
    .categories-button{
        border: none;
        background-color: var(--primary);
        color: var(--text);
        padding: 0.55rem;
        font-size: 1rem;
        width: 100%;

    }
</style>
