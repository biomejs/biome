---
"@biomejs/biome": patch
---

Fixed a Svelte parser error that incorrectly required a binding variable after `{:then}` and `{:catch}`. Biome now correctly accepts `{:then}` and `{:catch}` without a binding, as well as the `{#await expr then}` and `{#await expr catch}` shorthand forms.
