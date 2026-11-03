---
"@biomejs/biome": patch
---

Added the new nursery rule [`useConsistentBooleanName`](https://biomejs.dev/linter/rules/use-consistent-boolean-name/). It requires variables, parameters, and functions that hold or return a boolean to start with a prefix such as `is` or `has`, and reports names with those prefixes that don't hold or return a boolean.

```js
const completed = progress === 100;
const hasName = "Sindre";
```
