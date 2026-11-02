---
"@biomejs/biome": patch
---

Fixed [#12818](https://github.com/biomejs/biome/issues/12818): [`useFilenamingConvention`](https://biomejs.dev/linter/rules/use-filenaming-convention/) now reports a Vue, Svelte, Astro, or HTML file once when `html.experimentalFullSupportEnabled` is enabled. Previously, the file was reported once per template expression and once per `<script>` block.
