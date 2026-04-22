---
"@biomejs/biome": patch
---

Fixed [#9810](https://github.com/biomejs/biome/issues/9810): [`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) no longer misses widening from concrete object types, class instances, object literals, tuples, functions, and regular expressions to `: object`.

```ts
function f(): object { return { retry: true }; }
```
