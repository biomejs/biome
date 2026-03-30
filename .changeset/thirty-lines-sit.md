---
"@biomejs/biome": patch
---

Fixed Grit queries that use native Biome AST node names with the native field names that are in our `.ungram` grammar files. Queries such as `JsConditionalExpression(consequent = $cons, alternate = $alt)` now compile successfully in `biome search` and grit plugins.
