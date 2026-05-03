---
"@biomejs/biome": patch
---

[`useStringStartsEndsWith`](https://biomejs.dev/linter/rules/use-string-starts-ends-with/) now detects string index comparisons on object literal properties initialized with `as const`.

This comparison is now reported because `message.value` is inferred as a string literal:

```ts
const message = { value: "hello" as const };
message.value[0] === "h";
```
