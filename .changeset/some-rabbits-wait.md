---
"@biomejs/biome": patch
---

Added the nursery rule [`noUnsafeTypeAssertion`](https://biomejs.dev/linter/rules/no-unsafe-type-assertion/), which disallows TypeScript type assertions while allowing const assertions.

```ts
const value = input as SomeType;
```
