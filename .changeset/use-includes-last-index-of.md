---
"@biomejs/biome": minor
---

Expand `useIncludes` nursery rule to also detect `lastIndexOf()` comparisons against `-1`/`0`, suggesting `.includes()` instead.
