---
"@biomejs/biome": patch
---

Fixed [#8480](https://github.com/biomejs/biome/issues/8480): [`useDestructuring`](https://biomejs.dev/linter/rules/use-destructuring/) no longer suggests illegal destructuring for:

1. Plain assignments to pre-declared variables (e.g. `thing = obj.thing` where `thing` is already declared) — destructuring would require `({ thing } = obj)` which may not be a valid refactor.
2. Numeric index access on types with index signatures but no iterable protocol (e.g. `const x = obj[0]` where `obj: { [key: string]: string }`) — array destructuring `[x] = obj` would be a type error.
