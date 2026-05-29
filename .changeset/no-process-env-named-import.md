---
"@biomejs/biome": patch
---

Fixed [#10447](https://github.com/biomejs/biome/issues/10447): now the rule [`noProcessEnv`](https://biomejs.dev/linter/rules/no-process-env) detects the use of `env` when it's imported from `process` and `node:process`.
