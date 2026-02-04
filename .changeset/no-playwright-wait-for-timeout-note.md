---
"@biomejs/biome": patch
---

Added a note about web-first assertions to [`noPlaywrightWaitForTimeout`](https://biomejs.dev/linter/rules/no-playwright-wait-for-timeout/). The diagnostic now suggests using `expect(locator).toBeVisible()` which auto-waits for conditions.
