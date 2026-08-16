---
"@biomejs/biome": patch
---

Fixed [#11310](https://github.com/biomejs/biome/issues/11310): Restored the performance of [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) and [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) when analyzed expressions share deep imported type paths.
