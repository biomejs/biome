---
"@biomejs/biome": patch
---

Fixed false positives reported in [#11215](https://github.com/biomejs/biome/issues/11215): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) and [`noUnusedImports`](https://biomejs.dev/linter/rules/no-unused-imports/) no longer report Svelte bindings used only by shorthand `class:` or `style:` directives as unused.
