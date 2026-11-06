---
"@biomejs/biome": patch
---

Fixed [#12818](https://github.com/biomejs/biome/issues/12818): [`useFilenamingConvention`](https://biomejs.dev/linter/rules/use-filenaming-convention/) now reports a filename diagnostic once per HTML, Vue, Svelte, or Astro document instead of once per embedded JavaScript snippet.
