---
"@biomejs/biome": patch
---

Fixed [#11610](https://github.com/biomejs/biome/issues/11610), [#11611](https://github.com/biomejs/biome/issues/11611), [#11612](https://github.com/biomejs/biome/issues/11612), [#11615](https://github.com/biomejs/biome/issues/11615), and [#11616](https://github.com/biomejs/biome/issues/11616): Biome no longer fully infers an imported generic declaration just to apply its type arguments, restoring type-aware lint performance for large libraries such as Zod. This improves [`useRegexpExec`](https://biomejs.dev/linter/rules/use-regexp-exec), [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises), [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises), [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing), and [`noUnsafePlusOperands`](https://biomejs.dev/linter/rules/no-unsafe-plus-operands).
