---
"@biomejs/biome": patch
---

Fixed [#4928](https://github.com/biomejs/biome/issues/4928): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports a value declaration as unused when its merged namespace is referenced.
