<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { Editor } from "@tiptap/core";
    import { StarterKit } from "@tiptap/starter-kit";

    let element = $state<HTMLElement>();
    let editorState = $state<{ editor: Editor | null }>({ editor: null });

    onMount(() => {
        editorState.editor = new Editor({
            element: element,
            extensions: [StarterKit],
            content: "empty",
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
