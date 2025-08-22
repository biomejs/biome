---
"@biomejs/biome": patch
---

Fixed [#7205](https://github.com/biomejs/biome/issues/7205): The noDuplicateTestHooks rule now treats chained describe variants (e.g., describe.each/for/todo) as proper describe scopes, eliminating false positives.
