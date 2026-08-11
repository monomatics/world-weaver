<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { goto } from "$app/navigation";
    import { type World } from "../lib/dashboardUniversals.svelte";

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

    async function openSelectedWorld(){
        goto('/workspace')
        // something something goes here?
    }

    $effect(() => {
        listAllWorlds();
    });
</script>

<main>
    <div>
        {#if isLoadingWorld}
            loading worlds...
        {:else if worlds.length === 0}
            no worlds found.
        {:else}
            {#each worlds as world (world.id)}
                <button onclick={openSelectedWorld}>{world.name}</button>
            {/each}
        {/if}
    </div>
</main>
