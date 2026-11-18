/* should generate diagnostics */
import { writable } from "svelte/store";

const foo = writable(0);

export function track() {
	foo.subscribe(() => {});
}
