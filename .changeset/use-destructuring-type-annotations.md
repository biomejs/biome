---
"@biomejs/biome": patch
---

Fixed [#8478](https://github.com/biomejs/biome/issues/8478): [`useDestructuring`](https://biomejs.dev/linter/rules/use-destructuring/) no longer suggests destructuring for variable declarations with explicit type annotations. Previously, `const foo: string = object.foo` was incorrectly flagged, but destructuring to `const { foo } = object` would lose the `: string` annotation.
