<!-- should generate diagnostics -->
<script lang="ts">
	// `on` is a type-only import, which is erased at runtime: no fix.
	import { type on } from 'svelte/events';

	window.addEventListener('message', handler);
</script>
