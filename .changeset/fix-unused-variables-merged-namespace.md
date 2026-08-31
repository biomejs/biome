---
"@biomejs/biome": patch
---

Fixed [#11519](https://github.com/biomejs/biome/issues/11519): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports a value that merges with a namespace of the same name when that namespace is referenced. Previously the merged value was only spared when the namespace was exported.

Biome no longer reports `F` below:

```ts
function F() {}
namespace F {}
F();
```
