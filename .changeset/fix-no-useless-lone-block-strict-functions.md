---
"@biomejs/biome": patch
---

Fixed a false positive in [`noUselessLoneBlockStatements`](https://biomejs.dev/linter/rules/no-useless-lone-block-statements/) where blocks containing function declarations in strict scripts, strict functions, and class bodies were incorrectly reported as useless and unwrapped.
