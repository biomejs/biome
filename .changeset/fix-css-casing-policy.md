---
"@biomejs/biome": patch
---

Fixed CSS formatter casing for syntax-owned names while preserving author-defined names, including scoped keyframes and container scroll-state queries.

```diff
- A:HOVER { COLOR: INITIAL; }
+ A:hover { color: initial; }
- @KEYFRAMES :GLOBAL KeepFrames { FROM { COLOR: RED; } }
+ @keyframes :GLOBAL KeepFrames { from { color: RED; } }
- @CONTAINER scroll-state((SCROLLED: TOP) AND (STUCK)) { A:HOVER { COLOR: RED; } }
+ @container scroll-state((SCROLLED: TOP) AND (STUCK)) { A:hover { color: RED; } }
```
