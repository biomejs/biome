---
"@biomejs/biome": patch
---

Fixed [#12004](https://github.com/biomejs/biome/issues/12004): [`noDuplicateFontNames`](https://biomejs.dev/linter/rules/no-duplicate-font-names/) allows `font-family: monospace, monospace`, which preserves the inherited font size in browsers.
