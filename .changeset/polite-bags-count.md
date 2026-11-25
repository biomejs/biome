---
"@biomejs/biome": patch
---

Extended [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) to report `x ? x : y`, `!x ? y : x`, `if (!x) x = y`, `||=` on properties, `if (x == null)` whatever its type, and values typed `any` or `unknown`; `||` and its other forms now get a safe fix on arrays, `Date` and other built-in objects, functions, classes, and the `object` type; `ignorePrimitives` now ignores a type when any part of it is an ignored primitive, and no longer applies to comparisons with `null` or `undefined`.
