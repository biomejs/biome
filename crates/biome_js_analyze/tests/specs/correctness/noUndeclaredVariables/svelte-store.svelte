<script>
/* should not generate diagnostics */
import { writable } from "svelte";
let foo = writable(0);
$foo = 5;
</script>
