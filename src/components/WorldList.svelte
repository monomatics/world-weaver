<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";
    import { type World } from "../lib/dashboardUniversals.svelte";
    import { worldState } from "$lib/workspaceStates.svelte";

    let isLoadingWorld = $state(true);
    let worlds = $state<World[]>([]);

    async function listAllWorlds() {
        isLoadingWorld = true;
        try {
            worlds = await invoke("list_all_worlds");
        } catch (e) {
            console.error("Failed to load worlds:", e);
        } finally {
            isLoadingWorld = false;
        }
    }

    async function openSelectedWorld(id: string, name: string) {
        worldState.setWorldState(id, name);
        goto("/workspace");
    }

    $effect(() => {
        listAllWorlds();
    });
</script>

<main class="world-list-container">
    <div>
        {#if isLoadingWorld}
            loading worlds...
        {:else if worlds.length === 0}
            no worlds found.
        {:else}
            {#each worlds as world (world.id)}
                <div>
                    <button
                        class="world-list-button"
                        onclick={() => {
                            openSelectedWorld(world.id, world.name);
                        }}>{world.name}</button
                    >
                </div>
            {/each}
        {/if}
    </div>
</main>

<style>
    .world-list-container {
        background-color: var(--surface);
        padding: 0.5rem;
    }
    .world-list-button {
        border: none;
        color: var(--text);
        background-color: var(--overlay);
        font-size: 1rem;
        padding: 0.5rem;
        margin: 0.2rem;
        border-radius: 0.5rem;
    }
</style>
