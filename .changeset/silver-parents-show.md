---
"@biomejs/biome": patch
---

Fixed an issue where [`noUndeclaredClasses`](https://biomejs.dev/linter/rules/no-undeclared-classes) didn't correctly detect styles defined inside the Astro directive `is:global`.
