---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUnsafeReturn`](https://biomejs.dev/linter/rules/no-unsafe-return/), which disallows returning a value typed as `any` from a function.

```ts
function parse(text: string) {
  return JSON.parse(text); // reported
}
```
