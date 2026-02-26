---
"@biomejs/biome": patch
---

Fixed [#9115](https://github.com/biomejs/biome/issues/9115): The [`noPlaywrightMissingAwait`](https://biomejs.dev/linter/rules/no-playwright-missing-await/) rule no longer produces false positives on jest-dom matchers like `toBeVisible`, `toBeChecked`, `toHaveAttribute`, etc. For matchers shared between Playwright and jest-dom, the rule now checks whether `expect()`'s argument is a Playwright locator or page object before flagging. Added semantic variable resolution so that extracted Playwright locators (e.g. `const loc = page.locator('.item'); expect(loc).toBeVisible()`) are still correctly flagged.
