---
"@biomejs/biome": patch
---

Added the new nursery lint rule [`useExplicitReturnType`](https://biomejs.dev/linter/rules/use-explicit-return-type). It reports TypeScript functions and methods that omit an explicit return type.

```ts
function toString(x: any) { // rule triggered, it doesn't declare a return type
  return x.toString()
}
```

