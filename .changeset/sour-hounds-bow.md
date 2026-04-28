---
"@biomejs/biome": patch
---

Improved [`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) to detect `object` return annotations that hide built-in global class instances such as `Date`, `Map`, `Set`, `WeakMap`, and `Error`.
