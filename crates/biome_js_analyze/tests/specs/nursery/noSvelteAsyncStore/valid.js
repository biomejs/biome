/* should not generate diagnostics */
import { derived, readable, writable } from "svelte/store";
import { writable as otherWritable } from "./stores";

const a = writable(false, () => {});
const b = readable(false, function () {});
const c = derived(a, ($a) => $a);
const d = derived(a, ($a, set) => {
	fetchValue($a).then(set);
});
const e = writable(false);
const f = writable(async () => {});
const g = otherWritable(false, async () => {});

function shadowed(writable) {
	return writable(false, async () => {});
}

const h = get(false, async () => {});
