---
"@biomejs/biome": patch
---

Added support for negative value utilities in [`useSortedClasses`](https://biomejs.dev/linter/rules/use-sorted-classes/). Negative value utilities such as `-ml-2` or `-top-4` are now recognized and sorted correctly alongside their positive counterparts.

```jsx
// Now detected as unsorted:
<div class="-ml-2 p-4 -mt-1" />
// Suggested fix:
<div class="-mt-1 -ml-2 p-4" />
```
