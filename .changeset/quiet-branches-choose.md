---
"@biomejs/biome": patch
---

Type inference now evaluates conditional types such as `T extends U ? X : Y`, including `infer` and distribution over unions, instead of treating them as a union of both branches. This also applies to built-in types such as `Exclude`, `Extract`, `ReturnType`, `Parameters`, and `Awaited`, which improves the accuracy of type-aware rules such as [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/).

```ts
type Unwrap<T> = T extends Promise<infer U> ? U : T;
// Previously inferred as `unknown | Promise<number>`, now as `number`.
type Value = Unwrap<Promise<number>>;
```
