// should not generate diagnostics
import { SvelteDate } from "svelte/reactivity";

export function wrap($state) {
    return $state(new SvelteDate());
}
