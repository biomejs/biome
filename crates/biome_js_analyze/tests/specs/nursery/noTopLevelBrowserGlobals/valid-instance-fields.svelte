<!-- should not generate diagnostics -->
<script>
import { onMount } from "svelte";
class Client {
    width = window.innerWidth;
    #title = document.title;
}
onMount(() => new Client());
</script>
