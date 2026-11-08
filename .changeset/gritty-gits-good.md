---
"@biomejs/biome": patch
---

Fixed an issue where GritQL patterns wouldn't match snippets if they contain a trailing list separator.

For example, the pattern `` `a, b` `` wouldn't match the arguments in `function(a, b,)`, whereas now they do.
