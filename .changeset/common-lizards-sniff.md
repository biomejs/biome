---
"@biomejs/biome": patch
---

Added 11 new Playwright lint rules to the nursery (from eslint-plugin-playwright).

The following rules are now available:

- [`noPlaywrightMissingAwait`](https://biomejs.dev/linter/rules/no-playwright-missing-await/): Enforce awaiting async Playwright APIs.
- [`noPlaywrightElementHandle`](https://biomejs.dev/linter/rules/no-playwright-element-handle/): Prefer locators over element handles (`page.$()` and `page.$$()`).
- [`noPlaywrightEval`](https://biomejs.dev/linter/rules/no-playwright-eval/): Disallow `page.$eval()` and `page.$$eval()` methods.
- [`noPlaywrightForceOption`](https://biomejs.dev/linter/rules/no-playwright-force-option/): Disallow the `force` option on user interactions.
- [`noPlaywrightNetworkidle`](https://biomejs.dev/linter/rules/no-playwright-networkidle/): Disallow deprecated `networkidle` wait option.
- [`noPlaywrightPagePause`](https://biomejs.dev/linter/rules/no-playwright-page-pause/): Disallow `page.pause()` debugging calls in committed code.
- [`noPlaywrightUselessAwait`](https://biomejs.dev/linter/rules/no-playwright-useless-await/): Disallow unnecessary `await` on synchronous Playwright methods.
- [`usePlaywrightValidDescribeCallback`](https://biomejs.dev/linter/rules/use-playwright-valid-describe-callback/): Validate describe callback signatures are not async.
- [`noPlaywrightWaitForNavigation`](https://biomejs.dev/linter/rules/no-playwright-wait-for-navigation/): Prefer modern navigation APIs over deprecated `waitForNavigation()`.
- [`noPlaywrightWaitForSelector`](https://biomejs.dev/linter/rules/no-playwright-wait-for-selector/): Prefer locators over deprecated `waitForSelector()`.
- [`noPlaywrightWaitForTimeout`](https://biomejs.dev/linter/rules/no-playwright-wait-for-timeout/): Disallow hard-coded timeouts with `waitForTimeout()`.

Additionally, the existing `noFocusedTests` rule now detects Playwright's `test.only()` pattern, and `noSkippedTests` already handles Playwright's `test.skip()` pattern.
