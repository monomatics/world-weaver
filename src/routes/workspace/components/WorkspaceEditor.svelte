<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { Editor } from "@tiptap/core";
    import { StarterKit } from "@tiptap/starter-kit";
    import { Markdown } from "@tiptap/markdown";

    import { fetchEntity } from "$lib/workspaceStates.svelte";
    import { entityState } from "$lib/workspaceStates.svelte";

    let element = $state<HTMLElement>();
    let editorContent = entityState.savedEntity?.content;
    let editorState = $state<{ editor: Editor | null }>({ editor: null });

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
        });
    });
    onDestroy(() => {
        editorState.editor?.destroy();
    });
</script>

<main class="workspace-editor">
    <div class="workspace-editor" bind:this={element}></div>
</main>

<style>
    .workspace-editor {
        background-color: var(--surface);
        padding: 0.5rem;
        margin: 0.5rem;
        width: 60%;
    }
</style>
