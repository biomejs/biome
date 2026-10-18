---
"@biomejs/biome": patch
---

Fixed [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) reporting a declaration nested in a `namespace` that is itself inside `declare global` or inside an external module declaration. Both positions are documented as always ignored, and the rule offered a safe fix, so `biome check --write` renamed the declaration. This follows up [#11827](https://github.com/biomejs/biome/pull/11827).

```ts
export {}
declare global {
    namespace Outer {
        // no longer reported
        namespace my_NAMESPACE {}
    }
}
```
