---
"@biomejs/biome": patch
---

Fixed [#6891](https://github.com/biomejs/biome/issues/6891): Improved type inference for array indices.

**Example:**

```ts
const numbers: number[];
numbers[42] // This now infers to `number | undefined`.
```
