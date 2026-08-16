<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import {
        worldState,
        entityState,
        fetchEntity,
    } from "$lib/workspaceStates.svelte";

    let saveTimeout: ReturnType<typeof setTimeout>;
    let isMetadataSaving = $state("idle");

    $effect(() => {
        if (!entityState.savedEntityId) return;
        fetchEntity(entityState.savedEntityId);
    });

    $effect(() => {
        saveMetadata();
    });

    async function saveMetadata() {
        const currentEntityName = entityState.savedEntity?.name;
        const currentEntityCategory = entityState.savedEntity?.category;

        clearTimeout(saveTimeout);
        if (!currentEntityName || !currentEntityCategory) return;
        if (
            currentEntityName.trim() !== "" ||
            currentEntityCategory.trim() !== ""
        ) {
            isMetadataSaving = "typing...";
            saveTimeout = setTimeout(async () => {
                if (entityState.savedEntity == null) return;

                try {
                    isMetadataSaving = "saving...";
                    await invoke("update_entity", {
                        worldId: worldState.savedWorldId,
                        category: entityState.savedEntity.category,
                        entity: entityState.savedEntity,
                    });
                    isMetadataSaving = "saved";
                } catch (error) {
                    console.error("Failed to update entity:", error);
                    isMetadataSaving = "error";
                }
            }, 500);
        } else {
            isMetadataSaving = "idle";
        }
        return () => clearTimeout(saveTimeout);
    }
</script>

<main>
    {#if entityState.savedEntity}
        <b>Entity Metadata</b>
        <div>
            <b>Name:</b>
            <input
                class="metadata-input"
                bind:value={entityState.savedEntity.name}
            />
        </div>
        <div>
            <b>Category:</b>
            <input
                class="metadata-input"
                bind:value={entityState.savedEntity.category}
            />
        </div>
        <div>
            <b>ID:</b>
            {entityState.savedEntity.id}
        </div>
        <div>
            <b>custom metadata</b> <br />
            {#each Object.entries(entityState.savedEntity.metadata) as [key, value]}
                <span>{key}</span>
                <span>{value}</span>
            {:else}
            <div>nothing found..</div>
            {/each}
        </div>
    {:else}
        <div>No entity selected.</div>
    {/if}
</main>

<style>
    .metadata-input {
        border: none;
        color: var(--text);
        background-color: var(--surface);
    }
</style>
