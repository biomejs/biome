---
"@biomejs/biome": patch
---

Type inference now matches strings against template literal types in conditional types, including `infer` placeholders and `${number}` and `${bigint}` placeholders. This improves the accuracy of type-aware rules on code that parses strings at the type level.

```ts
type Split<S> = S extends `${infer T} as ${infer A}` ? [T, A] : never;
// Previously left undecided, now inferred as `["payments", "p"]`.
type Value = Split<"payments as p">;
```
