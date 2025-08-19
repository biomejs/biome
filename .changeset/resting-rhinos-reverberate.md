---
"@biomejs/biome": patch
---

Fixed [#6172](https://github.com/biomejs/biome/issues/6172): Resolved an issue with infering types for rest arguments. This issue caused the types of rest arguments to be incorrect and also caused sometimes extreme performance regressions in files that contained many methods with rest argument definitions.
