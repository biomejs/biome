---
"@biomejs/biome": patch
---

Improved the performance of GritQL plugins that use code snippets with metavariables, such as `` `process.env = $value` ``. Linting files with a lot of JSX text or string literals is significantly faster with such plugins.
