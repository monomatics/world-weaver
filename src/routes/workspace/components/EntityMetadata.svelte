<script lang="ts">
    import { homeDir } from "@tauri-apps/api/path";
    import { invoke, convertFileSrc } from "@tauri-apps/api/core";
    import {
        worldState,
        entityState,
        fetchEntity,
    } from "$lib/workspaceStates.svelte";

    type SaveStatus = "idle" | "typing" | "saving" | "saved" | "error";

    let saveStatus = $state<SaveStatus>("idle");
    let saveTimeout: ReturnType<typeof setTimeout>;
    let imageUrl = $state<string | null>(null);

    async function getImageUrl(relativePath: string): Promise<string | null> {
        const normalizedPath = relativePath.trim();
        if (!normalizedPath) return null;

        if (normalizedPath.startsWith("~/")) {
            const home = await homeDir();
            const fullPath = normalizedPath.replace("~/", home);
            return convertFileSrc(fullPath);
        }

        return convertFileSrc(normalizedPath);
    }

    $effect(() => {
        const path = entityState.savedEntity?.thumbnail_path;
        if (!path) {
            imageUrl = null;
            return;
        }

        let cancelled = false;

        (async () => {
            const url = await getImageUrl(path);
            if (!cancelled) {
                imageUrl = url;
            }
        })();

        return () => {
            cancelled = true;
        };
    });

    $effect(() => {
        const id = entityState.savedEntityId;
        if (!id) return;
        fetchEntity(id);
    });

    $effect(() => {
        const entity = entityState.savedEntity;
        if (!entity) {
            saveStatus = "idle";
            return;
        }

        const { id, name, category } = entity;
        const worldId = worldState.savedWorldId;

        clearTimeout(saveTimeout);

        const hasName = name?.trim() !== "";
        const hasCategory = category?.trim() !== "";
        if (!hasName || !hasCategory) {
            saveStatus = "idle";
            return;
        }

        saveStatus = "typing";
        saveTimeout = setTimeout(async () => {
            if (entityState.savedEntity?.id !== id) return;
            if (!worldState.savedWorldId) return;

            try {
                saveStatus = "saving";
                await invoke("update_entity", {
                    worldId: worldState.savedWorldId,
                    category: entityState.savedEntity.category,
                    entity: entityState.savedEntity,
                });
                saveStatus = "saved";
            } catch (error) {
                console.error("Failed to update entity:", error);
                saveStatus = "error";
            }
        }, 500);

        return () => clearTimeout(saveTimeout);
    });
</script>

<main>
    {#if entityState.savedEntity}
        <b>Entity Metadata</b>

        <div>
            <b>Image</b>
            <span>
                {#if imageUrl}
                    <img
                        src={imageUrl}
                        alt="Entity thumbnail"
                        style="max-width: 200px; display: block;"
                    />
                {:else}
                    <span>No image</span>
                {/if}
            </span>
        </div>

        <div>
            <b>Name:</b>
            <input
                class="metadata-input"
                bind:value={entityState.savedEntity.name}
                placeholder="Entity name"
            />
        </div>

        <div>
            <b>Category:</b>
            <input
                class="metadata-input"
                bind:value={entityState.savedEntity.category}
                placeholder="Category"
            />
        </div>

        <div>
            <b>ID:</b>
            {entityState.savedEntity.id}
        </div>

        <div>
            <b>Thumbnail Path:</b>
            <input
                class="metadata-input"
                bind:value={entityState.savedEntity.thumbnail_path}
                placeholder="Path to thumbnail file"
            />
        </div>

        <div>
            <b>Custom Metadata</b>
            {#if entityState.savedEntity.metadata && Object.keys(entityState.savedEntity.metadata).length > 0}
                {#each Object.keys(entityState.savedEntity.metadata) as key (key)}
                    <div class="metadata-row">
                        <span>{key}</span>
                        <input
                            class="metadata-input"
                            bind:value={entityState.savedEntity.metadata[key]}
                        />
                    </div>
                {/each}
            {:else}
                <div>No metadata found.</div>
            {/if}
        </div>

        <div class="save-indicator" data-status={saveStatus}>
            {#if saveStatus !== "idle"}
                {saveStatus}
            {/if}
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

    .metadata-row {
        display: flex;
        gap: 0.5rem;
        align-items: center;
        margin-bottom: 0.25rem;
    }

    .save-indicator {
        margin-top: 0.5rem;
        font-size: 0.875rem;
        height: 1.25rem;
        color: var(--text-secondary);
    }

    .save-indicator[data-status="saving"] {
        color: var(--accent, #3b82f6);
    }

    .save-indicator[data-status="saved"] {
        color: var(--success, #22c55e);
    }

    .save-indicator[data-status="error"] {
        color: var(--error, #ef4444);
    }
</style>
