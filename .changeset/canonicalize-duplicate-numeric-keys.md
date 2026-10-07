---
"@biomejs/biome": patch
---

Fixed [#11947](https://github.com/biomejs/biome/issues/11947): [`noDuplicateObjectKeys`](https://biomejs.dev/linter/rules/no-duplicate-object-keys/) and [`noDuplicateClassMembers`](https://biomejs.dev/linter/rules/no-duplicate-class-members/) now correctly canonicalize numeric and BigInt literal keys according to ECMAScript semantics (handling exponents, alternate radixes like hex/bin/octal, numeric separators, and trailing decimals).
