// should not generate diagnostics
import { writable } from "svelte/store";
const store = writable(0);
$store = 1;
