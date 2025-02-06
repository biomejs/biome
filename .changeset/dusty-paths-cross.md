---
"@biomejs/biome": minor
---

`useValidTypeof` now accepts comparisons with variables.

Previously, the rule required to compare a `typeof` expression against another `typeof` expression or a valid string literal.
We now accept more cases, notably comparison against a variable:

```js
if (typeof foo === bar) {
  // ...
}
```
