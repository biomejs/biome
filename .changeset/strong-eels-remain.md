---
"@biomejs/biome": patch
---

Added the `includeCaptures` option and the `captures` field to the experimental `searchPattern` workspace method. When `includeCaptures` is enabled, `captures` lists, for each match in `matches`, the GritQL variables that the match captured, such as `$message` in `` `console.log($message)` ``, with their names and source ranges. Most patterns that start from a code snippet or a syntax node report captures. Patterns that use `limit`, or that are only a variable such as `$x`, report none.
