---
"@biomejs/biome": patch
---

Fixed [#8759](https://github.com/biomejs/biome/issues/8759): The [`useConsistentTypeDefinitions`](https://biomejs.dev/linter/rules/use-consistent-type-definitions/) rule no longer converts empty object type declarations into interfaces, as it will conflict with the [`noEmptyInterface`](https://biomejs.dev/linter/rules/no-empty-interface/) rule and can cause an infinite loop when the both rules are enabled.
