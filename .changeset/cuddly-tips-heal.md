---
"@biomejs/biome": patch
---

Added the nursery rule [`noXorAsExponentiation`](https://biomejs.dev/linter/rules/no-xor-as-exponentiation/), which reports the bitwise XOR operator `^` between two decimal integer literals, where the exponentiation operator `**` was likely intended.

```js
const kibibyte = 2 ^ 10; // 8, not 1024
```
