<!-- should not generate diagnostics -->
<script>
import "./global.css";
import Child from "./invalid.svelte";
</script>

<style>
:root { --local-color: red; }
:global(:root) { --selector-global-color: blue; }
</style>

<Child />
