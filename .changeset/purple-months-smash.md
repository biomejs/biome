---
"@biomejs/biome": patch
---

Fixed [#13336](https://github.com/biomejs/biome/issues/13336): [`noInferrableTypes`](https://biomejs.dev/linter/rules/no-inferrable-types/) now allows literal type annotations on `const` declarations and `readonly` properties, including constructor parameter properties, because removing them can widen the type.

