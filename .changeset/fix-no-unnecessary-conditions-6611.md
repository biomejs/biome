---
"@biomejs/biome": patch
---

Fixed [#6611](https://github.com/biomejs/biome/issues/6611): the lint rule [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) now correctly handles several cases that were previously missed.

- Optional chaining on a non-nullish type is now flagged based on type information. Example: `function bar(arg: string) { return arg?.length }` now reports an unnecessary `?.`.
- Nullish coalescing on a non-nullish type is now flagged: `function bar(arg: string) { return arg ?? "default" }` now reports `??` as unnecessary.
- Logical expressions on member and call access use type information: `interface Foo { items: string[] }; function f(foo: Foo) { return foo.items || [] }` now reports `|| []` as unnecessary.
- Comparisons between a non-nullish type and `null`/`undefined` are flagged: `function f(x: string) { return x === null }` now reports.
- The `!expr` form of a condition now detects always-falsy: `const a = []; if (!a) {}` is now reported.
- `switch` statements are no longer wrongly flagged on the discriminant for literal unions like `'a' | 'b' | 'c'`. Instead, matching the behavior of `@typescript-eslint/no-unnecessary-condition`, individual `case` clauses that are statically unreachable — such as `case 'd':` inside `switch (value: 'a' | 'b' | 'c')` — are now flagged.
