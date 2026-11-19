---
"@biomejs/biome": patch
---

Improved type narrowing in type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/), [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/), and [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/). Narrowing now applies to `var` variables, imports, and variables of an enclosing function, and it continues into closures created after a check, as in TypeScript: always for a `const`, and for a parameter or a `let` that nothing assigns after the closure is created. Variables declared without a type, such as `let value;` or `const items = []`, now take their type from the values assigned or pushed to them.

For example, `noUnnecessaryConditions` now reports the optional chain below, because the closure is created after `timer` was checked:

```ts
function cancelLater(timer: Timer | null) {
  if (timer !== null) {
    return () => timer?.cancel();
  }
}
```
