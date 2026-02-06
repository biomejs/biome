---
"@biomejs/biome": patch
---

Fixed [#8478](https://github.com/biomejs/biome/issues/8478): [`useDestructuring`](https://biomejs.dev/linter/rules/use-destructuring/) no longer suggests destructuring when the variable has a type annotation, like `const foo: string = object.foo`.
