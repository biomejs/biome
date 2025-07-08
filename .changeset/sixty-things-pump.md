---
"@biomejs/biome": patch
---

Fixed [#6680](https://github.com/biomejs/biome/issues/6680): Biome incorrectly formatted container-style queries by inserting misplaced spaces.

```diff
- @container style (--responsive: true) {}
+ @container style(--responsive: true) {}
```

