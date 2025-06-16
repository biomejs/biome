---
"@biomejs/biome": patch
---

Fixed a false positive of `noUselessEscapeInRegex` where `\k` was reported as useless in non-Unicode regular expressions.
