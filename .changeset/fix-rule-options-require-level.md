---
"@biomejs/biome": patch
---

Fixed [#10674](https://github.com/biomejs/biome/issues/10674): a rule configuration that sets `options` without a `level` is now reported as an error when the configuration is parsed. Previously the missing `level` silently turned the rule off, so the rule stopped reporting with no warning.
