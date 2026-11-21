---
"@biomejs/biome": patch
---

Fixed [#8055](https://github.com/biomejs/biome/issues/8055): [`noConfusingVoidType`](https://biomejs.dev/linter/rules/no-confusing-void-type/) no longer reports `void` used as the constraint of a type parameter.

```ts
function printArg<T extends void>(arg: T) {}
```
