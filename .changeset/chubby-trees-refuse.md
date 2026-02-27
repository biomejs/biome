---
"@biomejs/biome": patch
---

Added the nursery rule [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/). This rule suggests using the nullish coalescing operator (`??`) instead of logical OR (`||`) when the left operand may be nullish. This prevents bugs where falsy values like `0`, `''`, or `false` are incorrectly treated as missing. Addresses [#8043](https://github.com/biomejs/biome/issues/8043)

```ts
// Invalid
declare const x: string | null;
const value = x || "default";

// Valid
const value = x ?? "default";
```
