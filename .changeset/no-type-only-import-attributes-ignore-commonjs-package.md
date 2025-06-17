---
"@biomejs/biome": major
---

Fixed [#5564](https://github.com/biomejs/biome/issues/5564). `noTypeOnlyImportAttributes` now ignores files ending with the extension `.ts` when the type field of `package.json` is set to `commonjs`.
