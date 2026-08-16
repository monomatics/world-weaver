<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";
    import { type World } from "../lib/dashboardUniversals.svelte";
    import { worldState } from "$lib/workspaceStates.svelte";
    import { Globe, FolderOpen } from "@lucide/svelte";

    let isLoadingWorld = $state(true);
    let errorMessage = $state<string | null>(null);
    let worlds = $state<World[]>([]);

    async function listAllWorlds() {
        isLoadingWorld = true;
        errorMessage = null;
        try {
            worlds = await invoke("list_all_worlds");
        } catch (e) {
            console.error("Failed to load worlds:", e);
            errorMessage = "Failed to load your worlds. Please try again." + e;
        } finally {
            isLoadingWorld = false;
        }
    }

    async function openSelectedWorld(world: World) {
        worldState.setWorldState(world.id, world.name);
        worldState.savedWorld = world;
        goto("/workspace");
    }

    // Initial load on mount
    $effect(() => {
        listAllWorlds();
    });
</script>

<div class="world-list-container">
    {#if isLoadingWorld}
        <div class="state-container">
            <p>Loading your worlds...</p>
        </div>
    {:else if errorMessage}
        <div class="state-container error">
            <p>{errorMessage}</p>
            <button class="retry-button" onclick={listAllWorlds}>Retry</button>
        </div>
    {:else if worlds.length === 0}
        <div class="state-container empty">
            <FolderOpen size={40} />
            <p class="empty-title">No worlds found</p>
            <span class="empty-subtitle">Create one above to get started!</span>
        </div>
    {:else}
        <div class="world-grid">
            {#each worlds as world (world.id)}
                <button
                    class="world-card"
                    onclick={() => openSelectedWorld(world)}
                >
                    <div class="world-card-icon">
                        <Globe size={24} />
                    </div>
                    <span class="world-name">{world.name}</span>
                    <div>{world.description}</div>
                </button>
            {/each}
        </div>
    {/if}
</div>

<style>
    .world-list-container {
        background-color: var(--overlay);
        border-radius: 1.5rem;
        min-height: 20rem;
        max-height: 40rem;
        overflow-y: auto;
        padding: 1.5rem;
    }

    .world-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
        gap: 1rem;
    }

    .world-card {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        justify-content: space-between;
        gap: 1rem;
        background-color: var(--primary);
        color: var(--text);
        border: none;
        border-radius: 1.25rem;
        padding: 1.25rem;
        font-size: 1rem;
        text-align: left;
        cursor: pointer;
        transition:
            transform 0.15s ease,
            background-color 0.2s ease,
            border-color 0.2s ease;
    }

    .world-card:hover {
        background-color: var(--surface-hover, #2a2a3d);
        border-color: var(--accent, #efb8c8);
        transform: translateY(-2px);
    }

    .world-card:active {
        transform: translateY(0);
    }

    .world-card-icon {
        background-color: rgba(239, 184, 200, 0.15);
        color: #efb8c8;
        padding: 0.6rem;
        border-radius: 0.75rem;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .world-name {
        font-weight: 700;
        word-break: break-word;
    }

    /* Status States (Loading, Empty, Error) */
    .state-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        min-height: 16rem;
        color: var(--text-muted, #a6adc8);
        gap: 0.75rem;
        text-align: center;
    }

    .empty-title {
        font-weight: 700;
        font-size: 1.1rem;
        margin: 0;
        color: var(--text);
    }

    .empty-subtitle {
        font-size: 0.9rem;
    }

    .retry-button {
        margin-top: 0.5rem;
        background-color: #efb8c8;
        color: #111;
        border: none;
        padding: 0.5rem 1rem;
        border-radius: 1rem;
        font-weight: 600;
        cursor: pointer;
    }

    :global(.spinner) {
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }
</style>
