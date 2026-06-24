---
"@biomejs/biome": patch
---

[`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) now suggests `??` for nullable values typed through a generic type alias.

```ts
type Maybe<T> = T | null;
declare const value: Maybe<string>;
const result = value || "default";
```
