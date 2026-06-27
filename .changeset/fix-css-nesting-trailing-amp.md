---
"@biomejs/biome": patch
---

Fixed [#7718](https://github.com/biomejs/biome/issues/7718): Biome now correctly parses CSS nesting selectors when `&` appears as a trailing sub-selector after a type selector, e.g. `h1& { color: red; }`.
