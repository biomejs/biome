---
"@biomejs/biome": patch
---

Fixed [#12956](https://github.com/biomejs/biome/issues/12956): [`useImportType`](https://biomejs.dev/linter/rules/use-import-type/) now distinguishes Astro template type and value uses, while [`useConst`](https://biomejs.dev/linter/rules/use-const/) and [`noVar`](https://biomejs.dev/linter/rules/no-var/) account for template writes. Cross-language references now prevent unsafe unused-variable rename suggestions.
