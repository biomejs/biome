---
"@biomejs/biome": patch
---

Added the nursery rule [`noArbitraryValue`](https://biomejs.dev/linter/rules/no-arbitrary-value/). Biome now reports Tailwind CSS arbitrary values such as `w-[400px]`, including in configured utility functions, tagged templates, and the default `class`/`className` attributes.
