---
"@biomejs/biome": patch
---

Improved type narrowing in type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/): assignments now narrow variables and properties, as in TypeScript. This covers `=`, compound and logical assignments such as `timer ??= make()`, `++` and `--`, `delete`, and destructuring assignments. A variable declared with a type and an initializer also starts with the initializer's type, so `const day: Day = "monday"` is `"monday"`.

For example, `noUnnecessaryConditions` now reports the optional chain below, because `timer` is a `Timer` after the assignment:

```ts
declare function make(): Timer;

function restart() {
  let timer: Timer | null = null;
  timer = make();
  timer?.cancel();
}
```
