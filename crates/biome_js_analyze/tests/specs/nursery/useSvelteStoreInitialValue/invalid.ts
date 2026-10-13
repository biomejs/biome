/* should generate diagnostics */
import { writable, readable, derived, type Readable } from "svelte/store";

export const w = writable<number>();
export const r = readable<string>();
export const d: Readable<number> = derived<Readable<number>, number>(w, ($w) => $w * 2);
