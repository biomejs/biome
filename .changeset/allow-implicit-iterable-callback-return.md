---
"@biomejs/biome": patch
---

Fixed [#9445](https://github.com/biomejs/biome/issues/9445): `useIterableCallbackReturn` now supports an `allowImplicit` option so callbacks can use `return;` when returning `undefined` intentionally.
