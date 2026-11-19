---
"@biomejs/biome": patch
---

Improved type inference and narrowing in type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) and [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/). Biome now types enums and their members, so comparisons and `switch` cases on enum members narrow, and `useExhaustiveSwitchCases` reports a `switch` that misses a member of an enum. Narrowing also applies to loose equality such as `value == "a"`, to comparisons with an annotated variable or with `true` and `false`, and through immediately invoked functions and class static blocks. A function such as `(value: unknown) => typeof value === "string"` now works as a type guard, as in TypeScript 5.5, also when it is passed to `filter`, `find`, or `every`, and `filter` with a callback that returns a `boolean` keeps the type of the array.

For example, `useExhaustiveSwitchCases` now reports the `switch` below, because it doesn't handle `Direction.Left`:

```ts
enum Direction {
  Up,
  Down,
  Left,
}

function move(direction: Direction) {
  switch (direction) {
    case Direction.Up:
      break;
    case Direction.Down:
      break;
  }
}
```
