---
"@biomejs/biome": patch
---

Fixed [#7920](https://github.com/biomejs/biome/issues/7920): The CSS parser, with Tailwind directives enabled, will no longer error when you use things like `prefix(tw)` in `@import` at rules.
