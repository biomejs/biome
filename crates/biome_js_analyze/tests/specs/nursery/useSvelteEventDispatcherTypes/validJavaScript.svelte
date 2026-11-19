<!-- should not generate diagnostics -->
<script>
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();
</script>

<button onclick={() => dispatch('save')}>Save</button>
