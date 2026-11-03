<!-- should not generate diagnostics -->
<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import * as svelte from 'svelte';
	import { createEventDispatcher as otherDispatcher } from './events';
	import * as events from './events';

	const dispatch = createEventDispatcher<{ save: string; close: null }>();
	const empty = createEventDispatcher<Record<string, never>>();
	const anything = createEventDispatcher<any>();
	const unknownEvents = createEventDispatcher<unknown>();
	const namespaced = svelte.createEventDispatcher<{ save: string }>();

	// Type syntax on the function instead of on the call.
	type Events = { save: string };
	const optional = createEventDispatcher<Events>?.();
	const parenthesized = (createEventDispatcher<Events>)();
	const casted = (createEventDispatcher as () => (name: 'save', detail: string) => void)();

	// Not a call.
	const reference = createEventDispatcher;

	// Other functions from `svelte`.
	onMount(() => {});
	svelte.onMount(() => {});

	// Functions with the same name that don't come from `svelte`.
	otherDispatcher();
	events.createEventDispatcher();
	const local = { createEventDispatcher() {} };
	local.createEventDispatcher();

	function scoped() {
		function createEventDispatcher() {}
		createEventDispatcher();
	}
</script>

<button onclick={() => dispatch('save', 'file')}>Save</button>
