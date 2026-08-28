---
"@biomejs/biome": patch
---

Fixed [#11519](https://github.com/biomejs/biome/issues/11519): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports a value as unused when it merges with a namespace of the same name that is used locally.
