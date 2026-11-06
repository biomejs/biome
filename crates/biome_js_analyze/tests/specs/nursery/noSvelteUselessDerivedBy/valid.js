/* should not generate diagnostics */
// Runes are only meaningful in Svelte components and `.svelte.js` modules.
const value = $derived.by(() => 1);
