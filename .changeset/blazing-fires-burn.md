---
"@biomejs/biome": patch
---
[noUndeclaredVariables](https://biomejs.dev/linter/rules/no-undeclared-variables/) is now able to bind read of value to a type-only import in ambient contexts ([#4526](https://github.com/biomejs/biome/issues/4526)).

In the following code, `A` is now correctly bound to the type-only import.
Previously, `A` was reported as an undeclared variable.

```ts
import type { A } from "mod";

declare class B extends A {}
```
