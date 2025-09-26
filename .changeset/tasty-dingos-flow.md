---
"@biomejs/biome": patch
---

Fixed [#7494](https://github.com/biomejs/biome/issues/7494): the parser now reports errors for missing elements in TypeScript generics.

```ts
// Before
type Foo<A, , B> = A & B;
type Foo = Bar<1, , 2>;

// After
type Foo<A, , B> = A & B;
//          ^ Expected a type parameter but instead found ','.
type Foo = Bar<1, , 2>;
//                ^ Expected a type parameter but instead found ','.
```
