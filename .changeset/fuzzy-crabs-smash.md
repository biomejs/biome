---
"@biomejs/biome": minor
---

Fixed [#8405](https://github.com/biomejs/biome/issues/8405): [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) now emits warnings/errors when a function returns union types such as `T | Promise<T>` which is used in conditionals.
