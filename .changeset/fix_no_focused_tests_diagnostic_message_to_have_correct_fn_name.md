---
"@biomejs/biome": patch
---

Fixed [#6380](https://github.com/biomejs/biome/issues/6380): The
`noFocusedTests` rule now correctly displays the function name in the diagnostic
message when a test is focused.

Every instance of a focused test function (like `fdescribe`, `fit`, `ftest` and
`only`) had the word 'only' hardcoded. This has been updated to use the actual
function name, so the message is now more accurate and specific.

Example for `fdescribe`:

```text
  i The 'fdescribe' method is often used for debugging or during implementation.

  i Consider removing 'f' prefix from 'fdescribe' to ensure all tests are executed.
```
