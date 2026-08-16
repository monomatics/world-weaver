<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import WorldList from "./components/WorldList.svelte";
    import { Plus, Check, X } from "@lucide/svelte";

    let isCreatingNewWorld = $state(false);
    let newWorldName = $state("");

    function startCreatingNewWorld() {
        newWorldName = "";
        isCreatingNewWorld = true;
    }

    function stopCreatingNewWorld() {
        isCreatingNewWorld = false;
        newWorldName = "";
    }

    async function createNewWorld() {
        if (!newWorldName.trim()) return;
        await invoke("create_new_world", { name: newWorldName.trim() });
        stopCreatingNewWorld();
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === "Escape") stopCreatingNewWorld();
        if (e.key === "Enter") createNewWorld();
    }

    function focusInput(node: HTMLInputElement) {
        node.focus();
    }

    // Custom Click Outside Action
    function clickOutside(node: HTMLElement, callback: () => void) {
        const handleClick = (event: MouseEvent) => {
            if (node && !node.contains(event.target as Node) && !event.defaultPrevented) {
                callback();
            }
        };

        document.addEventListener("click", handleClick, true);

        return {
            destroy() {
                document.removeEventListener("click", handleClick, true);
            }
        };
    }
</script>

<main class="dashboard-container">
    <header class="dashboard-header">
        <h1 class="dashboard-title">Your Worlds</h1>

        <div class="dashboard-actions">
            {#if !isCreatingNewWorld}
                <button
                    class="action-button primary"
                    onclick={startCreatingNewWorld}
                >
                    <span>Create a new world</span>
                    <Plus size={18} />
                </button>
            {:else}
                <!-- Attached use:clickOutside here -->
                <div
                    class="input-group"
                    use:clickOutside={stopCreatingNewWorld}
                >
                    <input
                        use:focusInput
                        bind:value={newWorldName}
                        onkeydown={handleKeydown}
                        placeholder="Enter world name..."
                        class="world-input"
                    />
                    <button class="icon-button" onclick={createNewWorld} title="Confirm">
                        <Check size={18} />
                    </button>
                    <button class="icon-button dangerous" onclick={stopCreatingNewWorld} title="Cancel">
                        <X size={18} />
                    </button>
                </div>
            {/if}
        </div>
    </header>

    <WorldList />
</main>
<style>
    .dashboard-container {
        padding: 1.5rem;
        max-width: 1200px;
        margin: 0 auto;
    }

    .dashboard-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 2rem;
        gap: 1rem;
    }

    .dashboard-title {
        font-size: 2rem;
        font-weight: 800;
        margin: 0;
        color: var(--text);
    }

    .action-button {
        display: inline-flex;
        align-items: center;
        gap: 0.5rem;
        background-color: var(--overlay);
        color: var(--text);
        border: 1px solid transparent;
        border-radius: 2rem;
        padding: 0.6rem 1.25rem;
        font-size: 0.95rem;
        font-weight: 600;
        cursor: pointer;
        transition: background-color 0.2s ease, transform 0.1s ease;
    }

    .action-button:hover {
        background-color: var(--surface);
    }

    .action-button:active {
        transform: scale(0.98);
    }

    .input-group {
        display: inline-flex;
        align-items: center;
        gap: 0.4rem;
        background-color: var(--overlay);
        border-radius: 2rem;
        padding: 0.3rem 0.4rem 0.3rem 1rem;
    }

    .world-input {
        background: transparent;
        border: none;
        outline: none;
        color: var(--text);
        font-size: 0.95rem;
        width: 180px;
    }

    .icon-button {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        background: transparent;
        border: none;
        color: var(--text);
        padding: 0.4rem;
        border-radius: 50%;
        cursor: pointer;
    }

    .icon-button:hover {
        background-color: var(--surface);
    }
</style>
