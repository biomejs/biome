---
"@biomejs/biome": patch
---

Fixed a performance regression in [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) that caused type inference to run repeatedly while linting a file.
