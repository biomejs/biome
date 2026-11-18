---
"@biomejs/biome": patch
---

Added the nursery rule [`noTestGlobals`](https://biomejs.dev/linter/rules/no-test-globals/), which reports Jest and Vitest APIs that are used as global variables instead of being imported.

```js
describe("sum", () => {});
```
