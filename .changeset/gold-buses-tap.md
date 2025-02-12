---
"@biomejs/biome": patch
---

Fix [#4982](https://github.com/biomejs/biome/issues/4982), js parser should throw a syntax error for `TsImportType` when syntax `import` without arguments after.

The follwing code will throw a syntax error:

```ts
type T = import;
type U = typeof import;
```