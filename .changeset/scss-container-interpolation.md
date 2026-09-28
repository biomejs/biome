---
"@biomejs/biome": patch
---

Fixed [#11822](https://github.com/biomejs/biome/issues/11822): the SCSS parser now supports interpolation in `@container` names and queries, such as `@container #{$name} (min-width: 400px) {}` and `@container #{$query} {}`.
