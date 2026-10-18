---
"@biomejs/biome": patch
---

Added the `captures` field to the results of the experimental `searchPattern` workspace method. For each match in `matches`, it lists the GritQL variables that the match captured, such as `$message` in `` `console.log($message)` ``, with their names and source ranges.
