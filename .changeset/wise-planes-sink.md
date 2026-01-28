---
"@biomejs/biome": patch
---

Added the rule [`noSolidEarlyReturn`](https://biomejs.dev/linter/rules/no-solid-early-return/) that disallows early returns and conditional return expressions in Solid components, which break reactivity since component functions only run once.
