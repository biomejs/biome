---
"@biomejs/biome": patch
---

Added the new nursery rule [`useConsistentIndexedObjectStyle`](https://biomejs.dev/linter/rules/use-consistent-indexed-object-style/), which requires one style for object types with arbitrary keys. By default, it requires the `Record` type instead of index signatures and mapped types.

```ts
// reported, prefer `Record<string, number>`
interface Scores {
  [name: string]: number;
}
```
