---
"@biomejs/biome": patch
---

Added a new nursery rule [`noNegationInEqualityCheck`](https://biomejs.dev/linter/rules/no-negation-in-equality-check/), which reports a negated left operand in a strict equality check (e.g. `!foo === bar`).
