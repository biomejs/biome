---
"@biomejs/biome": patch
---

[`useRegexpExec`](https://biomejs.dev/linter/rules/use-regexp-exec/) now detects `String#match()` calls on strings typed through a generic type alias.

```ts
type Id<T> = T;
declare const text: Id<string>;
text.match(/foo/);
```
