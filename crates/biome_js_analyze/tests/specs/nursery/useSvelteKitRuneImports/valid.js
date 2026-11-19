/* should not generate diagnostics */
import { page, navigating, updated } from "$app/state";
import { goto } from "$app/navigation";
import { writable } from "svelte/store";
import { page as other } from "$app/stores/page";
import { stores } from "./$app/stores";
