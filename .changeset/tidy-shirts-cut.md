---
"@biomejs/biome": patch
---

Updated `noImpliedEval` to flag `new Function()` usages, as its a form of indirect `eval`, and to include `no-new-func` as a rule source.
