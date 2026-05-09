---
"@biomejs/biome": patch
---

[`noUselessTypeConversion`](https://biomejs.dev/linter/rules/no-useless-type-conversion/) now detects redundant conversions on object literal properties initialized with `as const`.

This conversion is now reported because `message.value` is inferred as a string literal:

```ts
const message = { value: "text" as const };
String(message.value);
```
