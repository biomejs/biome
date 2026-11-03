---
"@biomejs/biome": patch
---

Fixed [#12766](https://github.com/biomejs/biome/issues/12766): [`usePlaywrightValidDescribeCallback`](https://biomejs.dev/linter/rules/use-playwright-valid-describe-callback/) no longer reports `test.describe(title, details, callback)`. The rule now validates the last argument as the callback, as the original ESLint rule does.

```js
test.describe("suite", { tag: "@smoke" }, () => {});
```
