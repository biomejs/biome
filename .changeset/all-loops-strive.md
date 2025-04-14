---
"@biomejs/biome": minor
---

The rule [`noFocusedTests`](https://biomejs.dev/linter/rules/no-focused-tests/) can now detect the usage of focused tests inside loops.

```js
// invalid
describe.only.each([["a"], ["b"]])("%s", (a) => {});
it.only.each([["a"], ["b"]])("%s", (a) => {});
test.only.each([["a"], ["b"]])("%s", (a) => {});

// valid
describe.each([["a"], ["b"]])("%s", (a) => {});
it.each([["a"], ["b"]])("%s", (a) => {});
test.each([["a"], ["b"]])("%s", (a) => {});
```
