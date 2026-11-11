---
"@biomejs/biome": patch
---

Added the new nursery rule [`noRedundantTypeConstituents`](https://biomejs.dev/linter/rules/no-redundant-type-constituents/), which reports members of union and intersection types that have no effect, such as a literal type next to its primitive type, or `never` in a union.

```ts
type Status = "active" | string;
```
