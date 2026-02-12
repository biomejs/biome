---
"@biomejs/biome": minor
---

Added the nursery rule [`noUnsafeAssignment`](https://biomejs.dev/linter/rules/no-unsafe-assignment/). This type-aware rule detects when a value typed as `any` is assigned to a variable, preventing `any` from silently spreading through the codebase. Inspired by typescript-eslint's `no-unsafe-assignment` rule.

```ts
declare function getPayload(): any;

// Flagged: `any` leaks into `payload`
const payload = getPayload();

// Allowed: explicit `unknown` is a safe alternative
const safe: unknown = getPayload();
```
