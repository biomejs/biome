---
"@biomejs/biome": patch
---

fix(grit): make $filename metavariable accessible in GritQL plugins

Fixed an issue where the `$filename` metavariable was not accessible when using `where` clauses in GritQL patterns. The fix separates global variables (like `$filename`) from local pattern variables into different scopes, preventing the auto-wrap bubble from resetting global variable values.
