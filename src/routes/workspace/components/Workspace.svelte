<script lang="ts">
    import { worldState } from "$lib/workspaceStates.svelte";
    import EntityPanel from "./EntityPanel.svelte";
    import EntityCategoryPanel from "./EntityCategoryPanel.svelte";
    import EntityMetadata from "./EntityMetadata.svelte";
    import WorkspaceEditor from "./WorkspaceEditor.svelte";
</script>

<main>
    <div class="workspace-label">{worldState.savedWorldId}</div>
    <div class="workspace-label">{worldState.savedWorldName}</div>
    <div style="display:flex;">
        <EntityCategoryPanel />
        <EntityPanel />
        <div class="workspace-editor">
            <EntityMetadata />
            <WorkspaceEditor />
        </div>
    </div>
</main>

<style>
    .workspace-label {
        background-color: var(--surface);
        padding: 0.2rem;
        margin: 0.4rem;
    }
    .workspace-editor {
        background-color: var(--surface);
        padding: 0.5rem;
        margin: 0.5rem;
        width: 60%;
    }
</style>
