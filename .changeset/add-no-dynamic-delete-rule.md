---
"@biomejs/biome": patch
---

Added the nursery rule [`noDynamicDelete`](https://biomejs.dev/linter/rules/no-dynamic-delete/), which disallows using `delete` with a property name that is computed at runtime. Fixed property names written as string or number literals are allowed.

```js
delete container[name];
```
