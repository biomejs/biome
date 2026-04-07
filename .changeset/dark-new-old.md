---
"@biomejs/biome": "patch"
---

Added the rule [`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/). The rule detects when a function's return type annotation is wider than what the implementation actually returns.

```ts
// Flagged: `: string` is wider than `"loading" | "idle"`
function getStatus(b: boolean): string {
  if (b) return "loading";
  return "idle";
}
```
