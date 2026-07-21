---
"@biomejs/biome": patch
---

Fixed false positives in [`noBaseToString`](https://biomejs.dev/linter/rules/no-base-to-string/) and [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) when member, stringification, or nullish inference cannot complete. These rules now suppress diagnostics instead of reporting from partial type information. For example, neither expression is reported when a recursive type cannot be fully resolved:

```ts
type Recursive = Recursive;
declare const value: Recursive;

String(value);
value || "fallback";
```
