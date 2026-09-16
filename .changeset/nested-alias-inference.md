---
"@biomejs/biome": patch
---

Fixed type inference through generic type aliases that instantiate another generic type with a nested generic argument, such as `type Nested<T> = Box<Wrapper<T>>`. Type-aware rules now resolve members of such types:

```ts
declare const nested: Nested<number>;
// noUnnecessaryConditions now reports that `??` is unnecessary.
const inner = nested.value.inner ?? 1;
```
