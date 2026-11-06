<!-- should generate diagnostics -->
<script>
	import { get_current_component } from "svelte/internal";
</script>

<p>{get_current_component()}</p>
