---
"@biomejs/biome": patch
---

Fixed [#6172](https://github.com/biomejs/biome/issues/6172): Resolved an issue with inferring types for rest parameters. This issue caused rest-parameter types to be incorrect, and in some cases caused extreme performance regressions in files that contained many methods with rest-parameter definitions.
