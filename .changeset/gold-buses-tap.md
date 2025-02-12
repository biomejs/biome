---
"@biomejs/biome": patch
---

Fix [#4982](https://github.com/biomejs/biome/issues/4982), the JavaScript parser now throws a syntax error for the following code:

```ts
type T = import;
type U = typeof import;
```