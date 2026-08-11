<script lang="ts">
    import { goto } from "$app/navigation";
    import Workspace from "./components/Workspace.svelte";
    import { House } from "@lucide/svelte";

    function returnHome() {
        goto("/");
    }
</script>

<main>
    <button onclick={returnHome}><House /></button>
    <Workspace />
</main>
