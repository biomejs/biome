---
"@biomejs/biome": patch
---

[`useIncludes`](https://biomejs.dev/linter/rules/use-includes/) now detects `indexOf` presence checks on values typed through a generic type alias.

```ts
type Id<T> = T;
declare const text: Id<string>;
text.indexOf("a") !== -1;
```
