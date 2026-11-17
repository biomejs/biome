---
"@biomejs/biome": patch
---

Improved type narrowing in type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) and [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/). Biome now narrows values in `switch` statements, `for...of` and `for...in` loops, `try`/`catch`/`finally` blocks, inside optional chains such as `user?.greet(user)`, and in comparisons with constants such as `const LOADED = "loaded"`. A `switch` that handles every possible value also narrows the code that follows it.

For example, `noUnnecessaryConditions` now reports the optional chain below, because `shape` can't be `undefined` in the `"circle"` case:

```ts
type Shape =
  | { kind: "circle"; radius: number }
  | { kind: "square"; size: number };

function radius(shape: Shape | undefined) {
  switch (shape?.kind) {
    case "circle":
      return shape?.radius;
  }
}
```

Biome now also widens literal types as TypeScript does: `let retries = 3` has the type `number` instead of `3`, so rules no longer report conditions such as `retries ? a : b` as always truthy.
