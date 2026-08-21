---
"@biomejs/biome": patch
---

Fixed [#11390](https://github.com/biomejs/biome/issues/11390): [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) no longer performs unnecessary type inference on call arguments when checking methods of non-generic class instances created with `new`.
