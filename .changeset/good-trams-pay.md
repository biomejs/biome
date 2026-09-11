---
"@biomejs/biome": patch
---

Fixed [#6888](https://github.com/biomejs/biome/issues/6888). GritQL plugins can now use `contains` on import-clause metavariables such as `$clause` in `import $clause from "module"` patterns.
