---
"@biomejs/biome": patch
---

Added [noMagicNumbers](https://github.com/biomejs/biome/issues/4333) nursery rule.
The rule detects and reports the use of "magic numbers" — numeric literals that are used directly in code without being assigned to a named constant.

Example:

```js
let total = price * 1.23; // Magic number for tax rate will highlight 1.23 as magic number
```
