---
"@biomejs/biome": patch
---

[`noUselessTypeConversion`](https://biomejs.dev/linter/rules/no-useless-type-conversion/) now detects redundant conversions when the value type comes from a generic type alias.

```ts
type Identity<T> = T;
declare const text: Identity<string>;
String(text);
```
