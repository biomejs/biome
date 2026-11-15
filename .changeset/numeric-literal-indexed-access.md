---
"@biomejs/biome": patch
---

Improved type inference for indexed access types with a number literal index, such as `[string, number][0]`, `Parameters<typeof f>[1]`, or `{ 0: "zero" }[0]`. Rules that rely on type information, such as [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/), now see the selected element or property type instead of an unknown type.
