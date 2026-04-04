---
"@biomejs/biome": patch
---

Added the new nursery rule [useConsistentTestIt](https://biomejs.dev/linter/rules/use-consistent-test-it/) in the `test` domain. The rule enforces consistent use of either `it` or `test` for test functions in Jest/Vitest suites, with separate control for top-level tests and tests inside `describe` blocks.

Invalid:

```js
test("should fly", () => {}); // Top-level test using 'test' flagged, convert to 'it'

describe('pig', () => {
  test("should fly", () => {}); // Test inside 'describe' using 'test' flagged, convert to 'it'
});
```
