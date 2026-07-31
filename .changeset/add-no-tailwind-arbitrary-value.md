---
"@biomejs/biome": patch
---

Added the nursery rule [`noTailwindArbitraryValue`](https://biomejs.dev/linter/rules/no-tailwind-arbitrary-value/). Biome now reports Tailwind CSS arbitrary values such as `w-[400px]`, including in HTML/JSX class attributes, configured utility functions, and tagged templates.
