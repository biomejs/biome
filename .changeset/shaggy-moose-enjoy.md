---
"@biomejs/biome": patch
---

Fixed the [`noDuplicateFontNames`](https://biomejs.dev/linter/rules/no-duplicate-font-names/) rule so it reports repeated generic family names in `font` shorthand values such as `font: 1em sans-serif, sans-serif`, while keeping quoted family names distinct from generic keywords.
