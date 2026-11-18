---
"@biomejs/biome": patch
---

Added the nursery rule [`useTopLevelDescribe`](https://biomejs.dev/linter/rules/use-top-level-describe/), which reports tests and hooks that are not inside a `describe` block. The `maxTopLevelDescribes` option also limits how many top-level `describe` blocks a file can have.

```js
test("adds two numbers", () => {});
```
