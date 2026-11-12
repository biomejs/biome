---
"@biomejs/biome": patch
---

Added the nursery rule [`noMisusedSpread`](https://biomejs.dev/linter/rules/no-misused-spread/), which uses type information to report spreads that produce unexpected results: strings spread into arrays or arguments, and promises, functions, maps, arrays, iterables, class instances, and classes spread into objects or JSX attributes.

```ts
declare function getUser(): Promise<{ name: string }>;
const user = { ...getUser() };
```
