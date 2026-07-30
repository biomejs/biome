---
"@biomejs/biome": patch
---

Fixed [#11087](https://github.com/biomejs/biome/issues/11087): [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) no longer reports optional chains and nullish coalescing whose receiver can be nullish.

For example, the optional chain and fallback in the following code are no longer reported:

```ts
declare const usage: { range: { startDate: string } } | null;
const startDate = usage?.range.startDate ?? "N/A";
```
