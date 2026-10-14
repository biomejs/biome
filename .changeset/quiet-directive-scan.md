---
"@biomejs/biome": patch
---

Improved the performance of [`noTsIgnore`](https://biomejs.dev/linter/rules/no-ts-ignore/). The rule now spends much less time in files that contain no `@ts-ignore` comment.
