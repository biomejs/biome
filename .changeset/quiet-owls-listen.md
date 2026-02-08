---
"@biomejs/biome": patch
---

Fixed [#8476](https://github.com/biomejs/biome/issues/8476).
[useAwaitThenable](https://biomejs.dev/linter/rules/use-await-thenable/) no longer reports false positives for `await` on call expressions whose return type cannot be resolved (e.g., cross-module function calls to Node.js builtins or npm packages).
