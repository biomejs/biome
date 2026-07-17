---
"@biomejs/biome": patch
---

[`noBaseToString`](https://biomejs.dev/linter/rules/no-base-to-string/) now detects default object stringification when the value type comes from a generic type alias.

```ts
type Id<T> = T;
declare const obj: Id<{ a: number }>;
String(obj);
```
