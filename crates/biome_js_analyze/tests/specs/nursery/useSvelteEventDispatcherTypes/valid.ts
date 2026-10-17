/* should not generate diagnostics */
import { createEventDispatcher } from 'svelte';

export function createDispatcher() {
	return createEventDispatcher();
}
