/* should generate diagnostics */
import { derived, readable, writable } from "svelte/store";
import { writable as createStore } from "svelte/store";
import * as stores from "svelte/store";

const a = writable(false, async () => {});
const b = readable(false, async () => {});
const c = derived(a, async ($a) => {});
const d = writable(false, async function () {});
const e = readable(false, async function* () {});
const f = writable(false, (async () => {}));
const g = createStore(false, async () => {});
const h = stores.writable(false, async () => {});
const i = stores["derived"](a, async ($a, set) => {});
