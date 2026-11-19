---
"@biomejs/biome": patch
---

Improved type narrowing in type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) and [`useStrictBooleanExpressions`](https://biomejs.dev/linter/rules/use-strict-boolean-expressions/). Biome now narrows `unknown` and `any` as TypeScript does: a truthy or non-nullish `unknown` is `{}`, `typeof value === "object"` gives `object | null`, and `any` narrows with `typeof`, `instanceof`, and type guards, also when it comes from `JSON.parse()`. A check on a generic value now gives an intersection such as `T & {}` or `T & string`, type guards that target `NonNullable<T>` narrow, and so do `asserts this is T`, `#field in value`, and `value.constructor === MyClass`. Intersection types such as `NonNullable<string | null>` and `(A | B) & C` now resolve as in TypeScript.

For example, `noUnnecessaryConditions` now reports the nullish coalescing below, because `value` can't be nullish there:

```ts
function fallback(value: unknown) {
  if (value != null) {
    return value ?? 1;
  }
}
```
