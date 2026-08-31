---
"@biomejs/biome": patch
---

Fixed [#11519](https://github.com/biomejs/biome/issues/11519): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports a function, class, or variable as unused when it shares a name with a TypeScript namespace that is used in the same file.

```ts
function F() {}
namespace F {}
F();
```
