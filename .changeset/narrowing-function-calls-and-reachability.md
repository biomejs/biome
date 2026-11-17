---
"@biomejs/biome": patch
---

Improved type narrowing in type-aware lint rules such as [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/), [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/), and [`useStrictBooleanExpressions`](https://biomejs.dev/linter/rules/use-strict-boolean-expressions/). Type guards and assertion functions now narrow when they are imported, called as methods, generic, or overloaded, including `Array.isArray` and `every`, and `values.filter(isString)` now has the type `string[]`. A call to a function that returns `never`, such as `fail()`, now ends the code path, and so does a branch that a literal `true` or `false` condition never takes.

For example, `noUnnecessaryConditions` now reports the optional chain below, because `fail()` never returns:

```ts
declare function fail(message: string): never;

function size(value: string | null) {
  if (value === null) fail("missing");
  return value?.length;
}
```

Biome now also includes `undefined` in the inferred return type of a function that can end without returning a value, as TypeScript does, so rules see that `label(false)` can be `undefined` with `function label(done: boolean) { if (done) return "done"; }`.
