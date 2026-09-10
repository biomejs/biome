---
"@biomejs/biome": minor
---

Added the nursery rule [`useValidTestTitle`](https://biomejs.dev/linter/rules/use-valid-test-title/).

This rule ports `valid-title` from `@vitest/eslint-plugin` and `eslint-plugin-jest`, enforcing valid titles for unit test cases and suites (`describe`, `test`, `it`, `suite`):
- Titles must not be empty.
- Titles must not have accidental leading or trailing whitespace (with safe auto-fix to trim).
- Titles must be string or template literals (configurable via `ignoreTypeOfDescribeName` and `ignoreTypeOfTestName`).
- Titles must not contain disallowed words (configurable via `disallowedWords`).
- Titles can be validated against regular expression patterns via `mustMatch` and `mustNotMatch`.
