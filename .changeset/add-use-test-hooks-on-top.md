---
"@biomejs/biome": patch
---

Added the nursery rule [`useTestHooksOnTop`](https://biomejs.dev/linter/rules/use-test-hooks-on-top) in the `test` domain. The rule flags lifecycle hooks (`beforeEach`, `beforeAll`, `afterEach`, `afterAll`) that appear after test cases in the same block, enforcing that hooks are defined before any test case.
