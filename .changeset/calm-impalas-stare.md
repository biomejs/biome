---
"@biomejs/biome": patch
---

Added a new nursery lint rule [`useExplicitReturnType`](https://biomejs.dev/linter/rules/use-explicit-return-type). The lint enforces every function to declare their return type.

```ts
function toString(x: any) { // rule triggered, it doesn't declare a return type
  return x.toString()
}
```

This rule was extracted from the function [`useExplicitType`](https://biomejs.dev/linter/rules/use-explicit-type), which we will sunsuet in favor of more specialized lint rules.

