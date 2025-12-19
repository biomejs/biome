---
"@biomejs/biome": patch
---

Fixed [#8494](https://github.com/biomejs/biome/issues/8494). Extended [`noUndeclaredEnvVars`](https://biomejs.dev/linter/rules/no-undeclared-env-vars/) to support bracket notation (`process.env["VAR"]`, `import.meta.env["VAR"]`), Bun runtime (`Bun.env.VAR`, `Bun.env["VAR"]`), and Deno runtime (`Deno.env.get("VAR")`).
