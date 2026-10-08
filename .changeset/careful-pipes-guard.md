---
"@biomejs/biome": patch
---

Added the nursery rule [`noAccidentalBitwiseOperators`](https://biomejs.dev/linter/rules/no-accidental-bitwise-operators/), which reports the bitwise operators `&`, `|`, and `|=` where the logical operators `&&`, `||`, and `||=` were likely intended.

```js
if (obj & obj.prop) {}
options = options | {};
```
