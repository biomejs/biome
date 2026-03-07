---
"@biomejs/biome": minor
---

Added the nursery rule [`useTestHooksInOrder`](https://biomejs.dev/linter/rules/use-test-hooks-in-order) in the `test` domain. The rule enforces that Jest/Vitest lifecycle hooks (`beforeAll`, `beforeEach`, `afterEach`, `afterAll`) are declared in the order they execute, making test setup and teardown easier to reason about.
