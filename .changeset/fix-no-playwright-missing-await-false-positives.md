---
"@biomejs/biome": patch
---

Fixed [#9115](https://github.com/biomejs/biome/issues/9115): The `noPlaywrightMissingAwait` rule no longer produces false positives on jest-dom matchers like `toBeVisible`, `toBeChecked`, `toHaveAttribute`, etc. For matchers shared between Playwright and jest-dom, the rule now checks whether `expect()`'s argument is a Playwright locator or page object before flagging.
