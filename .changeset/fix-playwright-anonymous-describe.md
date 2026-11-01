---
"@biomejs/biome": patch
---

Fixed [#12820](https://github.com/biomejs/biome/issues/12820): [`usePlaywrightValidDescribeCallback`](https://biomejs.dev/linter/rules/use-playwright-valid-describe-callback/) no longer reports an anonymous `test.describe(callback)` as a missing callback.

```js
test.describe(() => {
  test.use({ colorScheme: "dark" });
});
```
