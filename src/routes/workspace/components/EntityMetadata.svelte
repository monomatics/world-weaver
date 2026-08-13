<script lang="ts">
    import {
        entityState,
        fetchEntity,
    } from "$lib/workspaceStates.svelte";

    let isDirty = $state(false)

    $effect(() => {
        if (!entityState.savedEntityId) return;
        fetchEntity(entityState.savedEntityId);
    });
</script>

<main>
    {#if entityState.savedEntity}
        <b>Entity Metadata</b>
        <div>
            <b>Name:</b>
            <input class="metadata-input" bind:value={entityState.savedEntity.name} />
        </div>
        <div>
            <b>Category:</b>
            <input class="metadata-input" bind:value={entityState.savedEntity.category} />
        </div>
        <div>
            <b>ID:</b> {entityState.savedEntity.id}
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
