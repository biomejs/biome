---
"@biomejs/biome": patch
---

Added the nursery rule [`usePlaywrightValidDescribeCallback`](https://biomejs.dev/linter/rules/use-playwright-valid-describe-callback/). Validates that describe callback signatures are not async.

```js
test.describe('suite', async () => {});
```
