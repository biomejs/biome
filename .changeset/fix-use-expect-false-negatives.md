---
"@biomejs/biome": patch
---

Fixed [#9174](https://github.com/biomejs/biome/issues/9174): [`useExpect`](https://biomejs.dev/linter/rules/use-expect/) now correctly rejects Vitest [asymmetric matchers](https://vitest.dev/api/expect.html#expect-stringcontaining) like `expect.stringContaining()`, `expect.objectContaining()`, and utilities like `expect.extend()` that are not valid assertions. Previously these constructs caused false negatives, allowing tests without real assertions to pass the lint rule.
