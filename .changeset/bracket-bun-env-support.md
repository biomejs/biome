---
"@biomejs/biome": patch
---

Extended [`noUndeclaredEnvVars`](https://biomejs.dev/linter/rules/no-undeclared-env-vars/) to support bracket notation (`process.env["VAR"]`, `import.meta.env["VAR"]`) and Bun runtime (`Bun.env.VAR`, `Bun.env["VAR"]`). Fixes [#8494](https://github.com/biomejs/biome/issues/8494).
