---
"@biomejs/biome": patch
---

Fixed [#7905](https://github.com/biomejs/biome/issues/7905). Improved the accuracy of type-aware lint rules when analyzing re-exported functions and values.

Previously, when a binding was imported from another module, its type was not correctly inferred during the type analysis phase. This caused type-aware lint rules to fail to detect issues when working with re-exported imports.

The following rules now correctly handle re-exported imports:
- [`useAwaitThenable`](https://biomejs.dev/linter/rules/use-await-thenable/)
- [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/)
- [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/)
- [`useArraySortCompare`](https://biomejs.dev/linter/rules/use-array-sort-compare/)

Example of now-working detection:

```ts
// getValue.ts
export async function getValue(): Promise<number> {
    return 42;
}

// reexport.ts
export { getValue } from "./getValue";

// index.ts
import { getValue } from "./reexport";

// Previously: no diagnostic (type was unknown)
// Now: correctly detects that getValue() returns a Promise
await getValue(); // Valid - properly awaited
getValue();       // Diagnostic - floating promise
```
