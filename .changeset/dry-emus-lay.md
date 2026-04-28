---
"@biomejs/biome": patch
---

Fixed Vue `v-for` handling for [`noUndeclaredVariables`](https://biomejs.dev/linter/rules/no-undeclared-variables/) and [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/). Biome now recognizes variables declared by `v-for` directives and references to iterated values in Vue templates.
