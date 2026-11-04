---
"@biomejs/biome": patch
---

Added the nursery rule [`useConsistentTupleLabels`](https://biomejs.dev/linter/rules/use-consistent-tuple-labels/), which reports tuple type elements without a label when other elements of the same tuple have one.

```ts
type Point = [x: number, number];
```
