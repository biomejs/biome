---
"@biomejs/biome": patch
---

Fixed [#4517](https://github.com/biomejs/biome/issues/4517): [`noConfusingVoidType`](https://biomejs.dev/linter/rules/no-confusing-void-type/) no longer reports `void` in union types that contain only `void` and `never`, such as `void | never`.
