---
"@biomejs/biome": patch
---

Added support for mapped types such as `{ [K in keyof T]: T[K] }` to type inference. Type-aware rules now resolve the properties of mapped types, including generic aliases instantiated with concrete type arguments:

```ts
type Optional<T> = { [K in keyof T]?: T[K] };
declare const config: Optional<{ retries: number }>;
// `config.retries` is now inferred as `number | undefined`.
```

Mapped types that remap keys with an `as` clause remain unsupported.
