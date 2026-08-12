---
"@biomejs/biome": patch
---

Fixed [#11310](https://github.com/biomejs/biome/issues/11310): Resolved a performance regression in [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) when analyzing Promise-returning callbacks with imported call chains.
