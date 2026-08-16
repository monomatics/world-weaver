<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { Editor } from "@tiptap/core";
    import { StarterKit } from "@tiptap/starter-kit";
    import { Markdown } from "@tiptap/markdown";

    import { fetchEntity, worldState } from "$lib/workspaceStates.svelte";
    import { entityState } from "$lib/workspaceStates.svelte";

    let element = $state<HTMLElement>();
    let editorContent = entityState.savedEntity?.content;
    let editorState = $state<{ editor: Editor | null }>({ editor: null });
    let saveTimeout: ReturnType<typeof setTimeout>;

    $effect(() => {
        if (!entityState.savedEntityId) return;
        fetchEntity(entityState.savedEntityId);
    });

    onMount(() => {
        editorState.editor = new Editor({
            element: element,
            extensions: [StarterKit, Markdown],
            content: editorContent,
            contentType: "markdown",
            onTransaction: ({ editor }) => {
                editorState = { editor };
            },
            onUpdate: () => {
                entityState.isContentDirty = true;
                triggerAutoSave();
            },
        });
    });
    onDestroy(() => {
        editorState.editor?.destroy();
        clearTimeout(saveTimeout);
    });

    async function saveContent() {
        if (
            entityState.savedEntity == null ||
            !editorState.editor ||
            !entityState.isContentDirty ||
            entityState.isSavingContent
        )
            return;
        entityState.isSavingContent = true;
        const markdownContent = editorState.editor.getMarkdown();
        try {
            entityState.savedEntity.content = markdownContent;
            await invoke("update_entity", {
                worldId: worldState.savedWorldId,
                category: entityState.savedEntity.category,
                entity: entityState.savedEntity,
            });
            entityState.isContentDirty = false;
        } catch (error) {
            console.error("failed to save content:", error);
        } finally {
            entityState.isSavingContent = false;
        }
    }

    function triggerAutoSave() {
        clearTimeout(saveTimeout);
        saveTimeout = setTimeout(() => {
            saveContent();
        }, 2000);
    }
</script>

<main class="workspace-editor">
    <div class="workspace-editor" bind:this={element}></div>
</main>

<style>
    .workspace-editor {
        background-color: var(--secondary-container);
        padding: 0.5rem;
        margin: 0.5rem;
        width: 60%;
    }
</style>
