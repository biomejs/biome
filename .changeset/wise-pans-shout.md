---
"@biomejs/biome": patch
---

Added the nursery rule [`useBigintLiterals`](https://biomejs.dev/linter/rules/use-bigint-literals/), which enforces bigint literals over `BigInt()` calls with a literal argument.

```js
// Invalid
const bigint = BigInt(1);
const hex = BigInt("0xFF");

// Valid
const bigint = 1n;
const hex = 0xFFn;
const computed = BigInt(getSomeNumber());
```
