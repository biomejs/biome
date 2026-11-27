---
"@biomejs/biome": minor
---

Promoted [`useConsistentBooleanProps`](https://biomejs.dev/linter/rules/use-consistent-boolean-props/) from nursery to style. Run `biome migrate --write` to move existing nursery settings to style.

Deprecated [`noImplicitBoolean`](https://biomejs.dev/linter/rules/no-implicit-boolean/) for removal in the next major release. Use `useConsistentBooleanProps` with `mode: "explicit"` for the same style.
