---
"@biomejs/biome": patch
---

Fixed a parser panic reported in [#10708](https://github.com/biomejs/biome/issues/10708): Biome now recovers when unsupported CSS Modules `@value` rules or scoped `@keyframes` names end at EOF.
