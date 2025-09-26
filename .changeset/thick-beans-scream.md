---
"@biomejs/biome": patch
---

Fixed [#7451](https://github.com/biomejs/biome/issues/7451): the formatter now omits trailing commas in TypeScript type arguments.

```ts
// Before
type Foo = Bar<1, 2,>;

// After
type Foo = Bar<1, 2>;
```
