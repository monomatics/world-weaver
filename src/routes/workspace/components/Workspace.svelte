<script lang="ts">
    import { entityState, worldState } from "$lib/workspaceStates.svelte";
    import EntityPanel from "./EntityPanel.svelte";
    import EntityCategoryPanel from "./EntityCategoryPanel.svelte";
    import EntityMetadata from "./EntityMetadata.svelte";
    import WorkspaceEditor from "./WorkspaceEditor.svelte";
    import { goto } from "$app/navigation";
    import { House } from "@lucide/svelte";
    function returnHome() {
        goto("/");
    }
</script>

<main>
    <div class="workspace-label-container">
        <button class="workspace-button" onclick={returnHome}><House /></button>
        <div class="workspace-label">{worldState.savedWorldName}</div>
    </div>
    <div class="workspace-container">
        <EntityCategoryPanel />
        <EntityPanel />
        <div class="workspace-editor">
            <EntityMetadata />
            <WorkspaceEditor />
            <span class="status">
                {#if entityState.isSavingContent}
                    Saving...
                {:else if entityState.isContentDirty}
                    Unsaved changes
                {:else}
                    All changes saved
                {/if}
            </span>
        </div>
    </div>
</main>

<style>
    .workspace-label-container {
        display: flex;
        background-color: var(--surface);
        padding: 0.2rem;
        border-radius: 3rem;
    }
    .workspace-button {
        color: #efb8c8;
        padding: 1rem;
        background-color: transparent;
        border-radius: 3rem;
        align-items: center;
        border: none;
    }

    .workspace-button:hover {
        color: #2a2a3d;
        background-color: rgba(239, 184, 200, 0.15);
    }
    .workspace-label {
        font-weight: 600;
        border-radius: 3rem;
        padding: 1rem;
        margin: 0.4rem;
    }
    .workspace-container {
        display: flex;
    }
    .workspace-editor {
        background-color: var(--surface);
        margin: 0.5rem;
        padding: 1rem;
        border-radius: 2rem;
        height: max(40rem, 85vh);
        width: 70%;
    }
</style>
