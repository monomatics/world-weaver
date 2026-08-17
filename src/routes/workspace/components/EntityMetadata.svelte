<script lang="ts">
    import { homeDir } from "@tauri-apps/api/path";
    import { invoke, convertFileSrc } from "@tauri-apps/api/core";
    import {
        worldState,
        entityState,
        fetchEntity,
    } from "$lib/workspaceStates.svelte";
    import { open } from "@tauri-apps/plugin-dialog";

    type SaveStatus = "idle" | "typing" | "saving" | "saved" | "error";

    let saveStatus = $state<SaveStatus>("idle");
    let saveTimeout: ReturnType<typeof setTimeout>;
    let imageUrl = $state<string | null>(null);

    async function selectPicture() {
        const file = await open({
            multiple: false,
            directory: false,
            filters: [
                {
                    name: "Image",
                    extensions: ["png", "jpeg", "jpg", "webp", "gif", "svg"],
                },
            ],
        });

        if (file && entityState.savedEntity) {
            imageUrl = convertFileSrc(file);
            entityState.savedEntity.thumbnail_path = file;

            saveStatus = "typing";
            clearTimeout(saveTimeout);
            saveTimeout = setTimeout(async () => {
                if (!worldState.savedWorldId) return;
                if (entityState.savedEntity?.id == null) return;
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
        }
    }

    $effect(() => {
        const id = entityState.savedEntityId;
        if (!id) return;
        fetchEntity(id);
    });
    $effect(() => {
        const entity = entityState.savedEntity;
        if (entity?.thumbnail_path) {
            imageUrl = convertFileSrc(entity.thumbnail_path);
        } else {
            imageUrl = null;
        }
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

<main class="m3-card">
    {#if entityState.savedEntity}
        <header class="card-header">
            <h2 class="title-large">Entity Metadata</h2>
            {#if saveStatus !== "idle"}
                <span class="status-badge" data-status={saveStatus}>
                    {saveStatus}
                </span>
            {/if}
        </header>

        <section class="section">
            <div class="image-preview-container">
                {#if imageUrl}
                    <img src={imageUrl} alt="Entity thumbnail" class="entity-image" />
                {:else}
                    <div class="no-image-placeholder">No image selected</div>
                {/if}
                <button class="m3-button-filled" onclick={selectPicture}>
                    Choose picture
                </button>
            </div>
        </section>

        <section class="section form-grid">
            <div class="m3-text-field">
                <input
                    id="entity-name"
                    class="field-input"
                    bind:value={entityState.savedEntity.name}
                    placeholder=" "
                />
                <label for="entity-name" class="field-label">Name</label>
            </div>

            <div class="m3-text-field">
                <input
                    id="entity-category"
                    class="field-input"
                    bind:value={entityState.savedEntity.category}
                    placeholder=" "
                />
                <label for="entity-category" class="field-label">Category</label>
            </div>

            <div class="id-row">
                <span class="label-medium">ID:</span>
                <code class="id-badge">{entityState.savedEntity.id}</code>
            </div>
        </section>

        <section class="section">
            <h3 class="title-medium">Custom Metadata</h3>
            {#if entityState.savedEntity.metadata && Object.keys(entityState.savedEntity.metadata).length > 0}
                <div class="metadata-list">
                    {#each Object.keys(entityState.savedEntity.metadata) as key (key)}
                        <div class="metadata-row">
                            <span class="key-label">{key}</span>
                            <div class="m3-text-field">
                                <input
                                    class="field-input"
                                    bind:value={entityState.savedEntity.metadata[key]}
                                    placeholder="Value"
                                />
                            </div>
                        </div>
                    {/each}
                </div>
            {:else}
                <p class="body-medium empty-text">No custom metadata found.</p>
            {/if}
        </section>
    {:else}
        <div class="empty-state body-large">No entity selected.</div>
    {/if}
</main>

<style>
    :root {
        /* Material Design 3 Tokens mapped from your globals */
        --md-sys-color-surface: var(--surface, #1d1b20);
        --md-sys-color-surface-container: #2b2930;
        --md-sys-color-surface-variant: var(--overlay, #49454f);
        --md-sys-color-on-surface: var(--text, #e6e0e9);
        --md-sys-color-on-surface-variant: #cac4d0;
        --md-sys-color-primary: var(--primary, #d0bcff);
        --md-sys-color-on-primary: #381e72;
        --md-sys-color-outline: #938f99;
        --md-sys-color-outline-variant: #49454f;

        /* Status Colors */
        --md-sys-color-success: #a6f4c5;
        --md-sys-color-on-success: #00381e;
        --md-sys-color-error: #f2b8b5;
        --md-sys-color-on-error: #601410;

        /* Shape Tokens */
        --md-shape-corner-medium: 12px;
        --md-shape-corner-large: 16px;
        --md-shape-corner-full: 9999px;
    }

    .m3-card {
        background-color: var(--md-sys-color-surface-container);
        color: var(--md-sys-color-on-surface);
        border-radius: var(--md-shape-corner-large);
        padding: 1.5rem;
        max-width: 520px;
        display: flex;
        flex-direction: column;
        gap: 1.5rem;
        border: 1px solid var(--md-sys-color-outline-variant);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
        font-family: system-ui, -apple-system, sans-serif;
    }

    .card-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .title-large {
        font-size: 1.375rem;
        font-weight: 500;
        margin: 0;
    }

    .title-medium {
        font-size: 1rem;
        font-weight: 500;
        margin: 0 0 0.75rem 0;
    }

    .section {
        display: flex;
        flex-direction: column;
        gap: 1rem;
    }

    .form-grid {
        gap: 1.25rem;
    }

    /* M3 Outlined Text Field */
    .m3-text-field {
        position: relative;
        display: flex;
        flex-direction: column;
        width: 100%;
    }

    .field-input {
        width: 100%;
        box-sizing: border-box;
        background-color: transparent;
        color: var(--md-sys-color-on-surface);
        border: 1px solid var(--md-sys-color-outline);
        border-radius: 4px;
        padding: 0.875rem 1rem;
        font-size: 1rem;
        outline: none;
        transition: border-color 0.2s, border-width 0.1s;
    }

    .field-label {
        position: absolute;
        left: 0.875rem;
        top: 0.875rem;
        color: var(--md-sys-color-on-surface-variant);
        font-size: 1rem;
        pointer-events: none;
        background-color: var(--md-sys-color-surface-container);
        padding: 0 0.25rem;
        transition: transform 0.2s ease, font-size 0.2s ease, color 0.2s ease;
    }

    /* Floating label effects */
    .field-input:focus ~ .field-label,
    .field-input:not(:placeholder-shown) ~ .field-label {
        transform: translateY(-1.4rem);
        font-size: 0.75rem;
    }

    .field-input:focus {
        border: 2px solid var(--md-sys-color-primary);
        padding: 0.8125rem 0.9375rem; /* Keeps layout stable on 2px border */
    }

    .field-input:focus ~ .field-label {
        color: var(--md-sys-color-primary);
    }

    /* M3 Filled Button */
    .m3-button-filled {
        align-self: flex-start;
        background-color: var(--md-sys-color-primary);
        color: var(--md-sys-color-on-primary);
        border: none;
        border-radius: var(--md-shape-corner-full);
        padding: 0.625rem 1.5rem;
        font-size: 0.875rem;
        font-weight: 500;
        cursor: pointer;
        transition: background-color 0.2s, box-shadow 0.2s;
    }

    .m3-button-filled:hover {
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
        filter: brightness(1.08);
    }

    .m3-button-filled:active {
        filter: brightness(0.95);
    }

    /* Image Preview Section */
    .image-preview-container {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
        align-items: flex-start;
    }

    .entity-image {
        max-width: 100%;
        max-height: 200px;
        border-radius: var(--md-shape-corner-medium);
        object-fit: cover;
        border: 1px solid var(--md-sys-color-outline-variant);
    }

    .no-image-placeholder {
        width: 100%;
        height: 120px;
        display: flex;
        align-items: center;
        justify-content: center;
        background-color: var(--md-sys-color-surface-variant);
        border-radius: var(--md-shape-corner-medium);
        color: var(--md-sys-color-on-surface-variant);
        font-size: 0.875rem;
    }

    /* ID and Metadata Rows */
    .id-row {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .id-badge {
        background-color: var(--md-sys-color-surface-variant);
        padding: 0.25rem 0.5rem;
        border-radius: 4px;
        font-family: monospace;
        font-size: 0.875rem;
    }

    .metadata-list {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .metadata-row {
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .key-label {
        font-weight: 500;
        min-width: 80px;
        color: var(--md-sys-color-on-surface-variant);
        font-size: 0.875rem;
    }

    /* M3 Status Chips */
    .status-badge {
        font-size: 0.75rem;
        font-weight: 500;
        text-transform: capitalize;
        padding: 0.25rem 0.625rem;
        border-radius: var(--md-shape-corner-full);
        background-color: var(--md-sys-color-surface-variant);
        color: var(--md-sys-color-on-surface-variant);
    }

    .status-badge[data-status="saving"],
    .status-badge[data-status="typing"] {
        background-color: var(--md-sys-color-primary);
        color: var(--md-sys-color-on-primary);
    }

    .status-badge[data-status="saved"] {
        background-color: var(--md-sys-color-success);
        color: var(--md-sys-color-on-success);
    }

    .status-badge[data-status="error"] {
        background-color: var(--md-sys-color-error);
        color: var(--md-sys-color-on-error);
    }

    .empty-state, .empty-text {
        color: var(--md-sys-color-on-surface-variant);
    }
</style>