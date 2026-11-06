---
"@biomejs/biome": patch
---

Added the nursery rule [`noSharedBranchCode`](https://biomejs.dev/linter/rules/no-shared-branch-code/), which reports code that is repeated at the start or end of every branch of an `if` statement. That code runs no matter which branch is taken, so it can be moved before or after the `if` statement.

```js
if (condition) {
  doSomething();
  cleanup();
} else {
  doSomethingElse();
  cleanup();
}
```
