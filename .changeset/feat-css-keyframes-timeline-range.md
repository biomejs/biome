---
"@biomejs/biome": patch
---

Fixed [#7760](https://github.com/biomejs/biome/issues/7760): Added support for CSS scroll-driven animation `timeline-range-name` keyframe selectors (`cover`, `contain`, `entry`, `exit`, `entry-crossing`, `exit-crossing`). Biome no longer reports parse errors on keyframes like `entry 0% { ... }` or `exit 100% { ... }`.
