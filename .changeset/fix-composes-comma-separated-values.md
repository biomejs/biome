---
"@biomejs/biome": patch
---

Fixed [#10563](https://github.com/biomejs/biome/issues/10563): Biome now parses comma-separated CSS Modules `composes` values, such as `composes: classA from "./a.css", classB from "./b.css";`.
