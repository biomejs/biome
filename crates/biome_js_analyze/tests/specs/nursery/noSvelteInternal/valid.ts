/* should not generate diagnostics */
import type { Component } from "svelte";
declare module "svelte/internal" {
	export const noop: () => void;
}
