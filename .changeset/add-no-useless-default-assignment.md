---
"@biomejs/biome": patch
---

Added the nursery rule [`noUselessDefaultAssignment`](https://biomejs.dev/linter/rules/no-useless-default-assignment/), which uses type information to report default values that can never be used, because the value they replace can't be `undefined`.

```ts
function greet({ name = "friend" }: { name: string }) {} // reported
```
