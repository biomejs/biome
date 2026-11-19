<!-- should generate diagnostics -->
<script lang="ts">
	import { on } from 'svelte/events';

	const handler = (ev: Event) => {
		console.log(ev);
	};

	(window as any).addEventListener('message', handler);
</script>
