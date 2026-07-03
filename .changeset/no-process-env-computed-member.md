---
"@biomejs/biome": patch
---

Fixed [#10619](https://github.com/biomejs/biome/issues/10619): [`noProcessEnv`](https://biomejs.dev/linter/rules/no-process-env/) now also reports computed (bracket) member access. Previously only dot access was checked, so `process["env"]` and `env["NODE_ENV"]` (where `env` is imported from `node:process`) were missed. Both static and computed accesses are now reported.
