---
"@biomejs/biome": patch
---

Added the nursery rule [`noPlaywrightSkippedTest`](https://biomejs.dev/linter/rules/no-playwright-skipped-test/), which disallows usage of `.skip` and `.fixme` annotations in Playwright tests. Skipped tests are discouraged because they might be forgotten and remain disabled permanently.
