<!-- should generate diagnostics -->
<script lang="ts">
	import { createEventDispatcher, createEventDispatcher as createDispatcher } from 'svelte';
	import * as svelte from 'svelte';

	const dispatch = createEventDispatcher();
	const aliased = createDispatcher();
	const namespaced = svelte.createEventDispatcher();
	const computed = svelte['createEventDispatcher']();
	const parenthesized = (createEventDispatcher)();
	const optional = createEventDispatcher?.();
	const casted = createEventDispatcher() as (name: string) => void;

	function setup() {
		return createEventDispatcher();
	}
</script>

<button onclick={() => dispatch('save')}>Save</button>
