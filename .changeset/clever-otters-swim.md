---
"@biomejs/biome": patch
---

[`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) no longer misses widening from concrete object types, class instances, object literals, tuples, functions, and regular expressions to `: object`.

```ts
function f(): object { return { retry: true }; }
```
