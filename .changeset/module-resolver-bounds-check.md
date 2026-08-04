---
"@biomejs/biome": patch
---

Hardened the module resolver against the panic class behind [#10885](https://github.com/biomejs/biome/issues/10885): an out-of-bounds module-0 type reference now degrades to an unknown type (with a `debug_assert!` in debug builds) instead of killing the workspace worker with `index out of bounds`.
