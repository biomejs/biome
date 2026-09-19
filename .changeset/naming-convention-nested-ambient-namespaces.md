---
"@biomejs/biome": patch
---

[`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer reports a declaration nested in a `namespace` that is itself inside `declare global` or inside an external module declaration. Both positions are documented as always ignored, and the rule offers a safe fix, so `biome check --write` renames the declaration:

```ts
export {}
declare global {
    namespace Outer {
        // no longer reported
        namespace my_NAMESPACE {}
    }
}
```
