---
"@biomejs/biome": patch
---

Fixed [#6796](https://github.com/biomejs/biome/issues/6796): `noFloatingPromises` will no longer suggest to add `await` keyword inside synchronous callbacks nested inside `async` functions.
