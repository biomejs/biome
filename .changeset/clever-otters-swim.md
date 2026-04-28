---
"@biomejs/biome": patch
---

[`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) no longer misses widening from concrete object types, class instances, object literals, tuples, functions, and regular expressions to `: object`.

A function annotated `: object` returning an object literal:

```ts
function f(): object { return { retry: true }; }
```
