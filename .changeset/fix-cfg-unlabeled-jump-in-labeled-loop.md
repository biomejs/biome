---
"@biomejs/biome": patch
---

Fixed [`noUnreachable`](https://biomejs.dev/linter/rules/no-unreachable/), [`noUnreachableSuper`](https://biomejs.dev/linter/rules/no-unreachable-super/), [`useGetterReturn`](https://biomejs.dev/linter/rules/use-getter-return/), [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return/) and [`noFallthroughSwitchClause`](https://biomejs.dev/linter/rules/no-fallthrough-switch-clause/) skipping functions that contain an unlabeled `break` or `continue` inside a labeled loop or `switch`.

For example, `noUnreachable` now reports `afterBreak()` here:

```js
function f(xs) {
  outer: for (const x of xs) {
    break;
    afterBreak();
  }
}
```
