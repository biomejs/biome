---
"@biomejs/biome": patch
---

Fixed the CSS parser rejecting comma-separated selector lists inside `:global()` and `:local()` pseudo-class functions. Biome now correctly parses `:global(.foo, .bar)`.
