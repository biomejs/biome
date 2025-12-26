---
"@biomejs/biome": patch
---

Fixed [#8527](https://github.com/biomejs/biome/issues/8527): Improved type inference where analyzing code with repeated object property access and assignments (e.g. `node = node.parent`, a pattern common when traversing trees in a while loop) could hit an internal type limit. Biome now handles these cases without exceeding the type limit.
