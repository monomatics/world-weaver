<script lang="ts">
    import { worldState } from "$lib/workspaceStates.svelte";
    import EntityPanel from "./EntityPanel.svelte";
    import WorkspaceEditor from "./WorkspaceEditor.svelte";
</script>

<main>
    <div class="workspace-label">{worldState.savedWorldId}</div>
    <div class="workspace-label">{worldState.savedWorldName}</div>
    <div style="display:flex;">
        <EntityPanel />
        <WorkspaceEditor />
    </div>
</main>

<style>
    .workspace-label{
        background-color: var(--surface);
        padding: 0.2rem;
        margin: 0.4rem;
    }
</style>