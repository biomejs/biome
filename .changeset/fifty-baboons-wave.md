---
"@biomejs/biome": patch
---

Fixed [#11129](https://github.com/biomejs/biome/issues/11129): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports Vue bindings as unused when they are assigned through automatically unwrapped template refs.
