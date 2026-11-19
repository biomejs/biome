---
"@biomejs/biome": patch
---

Improved type narrowing in type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) and [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/). Biome now narrows a value through its discriminant property, also when the discriminant was copied into a `const` or destructured, through the `in` and `instanceof` operators, and through type guards that check for an object, a class, or an interface. Biome also merges interfaces and classes declared with the same name, and applies `declare module` and `declare global` augmentations, so rules see all their members.

For example, `noUnnecessaryConditions` now reports the optional chain below, because `result.data` can't be `null` once `result.state` is `"loaded"`:

```ts
type Result =
  | { state: "loaded"; data: { size: number } }
  | { state: "failed"; data: null };

function size(result: Result) {
  if (result.state === "loaded") {
    return result.data?.size;
  }
}
```

Rules also report fewer false positives on `catch` variables, which Biome now types as `unknown`, and on conditional expressions, whose type now always includes both branches.
