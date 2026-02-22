---
"@biomejs/biome": patch
---

Fixed [#8265](https://github.com/biomejs/biome/issues/8265): Biome now correctly detects test framework calls with optional TestOptions arguments (e.g., `describe("foo", { retry: 2 }, () => {})`). This fixes both formatting and the `noDuplicateTestHooks` lint rule for test frameworks like Vitest.
