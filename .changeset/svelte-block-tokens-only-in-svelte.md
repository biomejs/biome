---
"@biomejs/biome": patch
---

Fixed `{#`, `{/`, `{:` and `{@` being read as Svelte block openings in every HTML-like file. They are now Svelte-only, so in HTML, Vue and Angular files a sequence such as `{#if x}` is ordinary text instead of a parse error.
