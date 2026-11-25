---
"@biomejs/biome": patch
---

Fixed a bug where the Biome Scanner would index the same dependencies in `node_modules` more than once. This would result a slightly faster scanner.
