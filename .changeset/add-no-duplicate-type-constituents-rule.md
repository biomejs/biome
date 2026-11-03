---
"@biomejs/biome": patch
---

Added the nursery rule [`noDuplicateTypeConstituents`](https://biomejs.dev/linter/rules/no-duplicate-type-constituents/), which reports types listed more than once in a union or intersection type, and `undefined` in the type of an optional parameter.

```ts
type T = string | number | string;
function f(a?: string | undefined) {}
```
