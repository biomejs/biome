---
"@biomejs/biome": patch
---

Added the new nursery rule [`noDanglingUnderscore`](https://biomejs.dev/linter/rules/no-dangling-underscore/), which disallows variable names, function names, and property accesses that start or end with an underscore.

```js
const _count = 0; // reported
user._id; // reported
```
