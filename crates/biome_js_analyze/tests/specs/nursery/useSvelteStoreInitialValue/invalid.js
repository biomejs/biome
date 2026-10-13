/* should generate diagnostics */
import { writable, readable, derived } from "svelte/store";
import { writable as createWritable } from "svelte/store";
import * as store from "svelte/store";

export const w = writable();
export const r = readable();
export const d = derived([a, b], () => {});

// Too few arguments for `derived()`
const d2 = derived(a);
const d3 = derived();

// Aliased and namespace imports
const aliased = createWritable();
const ns1 = store.writable();
const ns2 = store.readable();
const ns3 = store.derived(a, ($a) => $a * 2);

const parenthesized = (writable)();
const optionalCall = writable?.();

const multiline = derived(
	[a, b],
	([$a, $b], set) => {
		set($a + $b);
	},
);

function createStore() {
	return writable();
}
