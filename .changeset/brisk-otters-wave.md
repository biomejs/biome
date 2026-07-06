---
"@biomejs/biome": patch
---

[`useArrayFind`](https://biomejs.dev/linter/rules/use-array-find/) now recognizes a first-element index supplied through a generic type alias.

```ts
type Index<T extends number> = T;
const first: Index<0> = 0;
[1, 2, 3].filter(x => x > 1)[first];
```
