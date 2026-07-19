---
"@biomejs/biome": patch
---

Fixed [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/): The rule now reports Promise-returning callbacks where a synchronous callback is expected when calls use tuple spreads or tuple rest parameters, including generic and deeply nested tuples, and when constructor signatures come from interface or object types. Recursive or excessively nested tuple spreads use a conservative fallback so analysis terminates.

For example, the following callback is now reported.

```ts
declare function consume(...args: [number, () => void]): void;
const prefix: [number] = [1];

consume(...prefix, async () => {});
```
