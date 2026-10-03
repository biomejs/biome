---
"@biomejs/biome": patch
---

Added the nursery rule [`noObjectConstructor`](https://biomejs.dev/linter/rules/no-object-constructor/), which disallows calling the `Object` constructor without arguments, such as `new Object()` or `Object()`. Biome can replace these calls with the object literal `{}`.
