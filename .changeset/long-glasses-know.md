---
"@biomejs/biome": patch
---

Fixed [#6633](https://github.com/biomejs/biome/6633). The `noImplicitCoercion` rule no longer reports diagnostics for `1 / value` expressions.

```js
1 / value // no error
```
