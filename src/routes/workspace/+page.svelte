<script lang="ts">
    import { goto } from "$app/navigation";
    import Workspace from "./components/Workspace.svelte";

    function returnHome() {
        goto("/");
    }
</script>

<main>
    <button onclick={returnHome}>home</button>
    <Workspace />
</main>
