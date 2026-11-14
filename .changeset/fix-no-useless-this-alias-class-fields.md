---
"@biomejs/biome": patch
---

Fixed [#13125](https://github.com/biomejs/biome/issues/13125): `noUselessThisAlias` no longer flags `this` aliases that are referenced inside class property or field initializers where `this` binds to the constructed instance or class constructor.
