<!-- should not generate diagnostics -->
<script>
import { onMount } from "svelte";
onMount(() => { document.title; });
$effect(() => { window.innerWidth; });
</script>
<button onclick={() => window.alert("hello")}>Open</button>
