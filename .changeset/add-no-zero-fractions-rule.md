---
"@biomejs/biome": patch
---

Added the new nursery rule [`noZeroFractions`](https://biomejs.dev/linter/rules/no-zero-fractions/), which disallows numeric literals with redundant zero fractions or dangling dots such as `1.0` and `1.`. Biome can now simplify these literals to shorter equivalents.
