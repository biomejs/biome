---
"@biomejs/biome": patch
---

[`useStringStartsEndsWith`](https://biomejs.dev/linter/rules/use-string-starts-ends-with/) now detects prefix and suffix index comparisons on strings typed through a generic type alias.

```ts
type Id<T> = T;
declare const text: Id<string>;
text[0] === "a";
```
