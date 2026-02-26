---
"@biomejs/biome": patch
---

Fixed [#8265](https://github.com/biomejs/biome/issues/8265): Biome now correctly detects test framework calls that use three arguments (label, options, callback) (e.g., `describe("foo", { retry: 2 }, () => {})`). This fixes both formatting and the [`noDuplicateTestHooks`](https://biomejs.dev/linter/rules/no-duplicate-test-hooks/) lint rule for test frameworks like Vitest.
