---
"@biomejs/biome": patch
---

Improved the diagnostics of [`noIdenticalTestTitle`](https://biomejs.dev/linter/rules/no-identical-test-title/). Biome now reports one diagnostic per repeated title, pointing at the first occurrence and at every later occurrence, instead of one diagnostic per duplicate.
