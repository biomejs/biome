---
"@biomejs/biome": patch
---

Added the nursery rule [useConsistentFunctionStyle](https://biomejs.dev/linter/rules/use-consistent-function-style/), which enforces function expressions or declarations and defaults to expressions. Variables with TypeScript type annotations are always allowed, and the `allowArrowFunctions` option permits arrows in declaration mode. Migration from ESLint's `func-style` copies the style and arrow-function option; named-export overrides are not supported.

```js
function greet() {
    return "Hello";
}
```
