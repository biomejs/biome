---
"@biomejs/biome": patch
---

Biome's type inference now classifies a value of an interface type as always truthy, as it already did for object and class types, following TypeScript's narrowing semantics.

A logical expression whose left side is such a value now short-circuits to that value's type, so [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) no longer reports the right-hand side below:

```ts
interface Foo {}
declare let foo: Foo;
foo || Promise.reject("no longer reported");
```
