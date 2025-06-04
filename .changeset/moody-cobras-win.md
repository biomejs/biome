---
"@biomejs/biome": patch
---

Fixed [#6144](https://github.com/biomejs/biome/issues/6144): [noUnusedImports](https://biomejs.dev/linter/rules/no-unused-imports/) reported incorrectly imports that were used as the type of parameters with the same name.
In the following code, the import `name` was reported as unused.

```ts
import name from "mod";
function f(name: name.Readable): void {}
```
