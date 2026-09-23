---
"@biomejs/biome": patch
---

Fixed [#11880](https://github.com/biomejs/biome/issues/11880): Biome no longer misparses a `<<` expression followed by a later `>>>` as TypeScript type arguments. For example, `const mask = 1 << bits` followed by `const m = mask >>> 0` on the next line now parses correctly.
