---
"@biomejs/biome": patch
---

Added the nursery rule [`useConsistentDefaultExportStyle`](https://biomejs.dev/linter/rules/use-consistent-default-export-style/), which enforces whether default-exported functions and classes are declared inside the `export default` statement or declared separately and exported by name.

```js
function foo() {}
export default foo; // reported
```
