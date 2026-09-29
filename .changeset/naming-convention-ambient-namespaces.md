---
"@biomejs/biome": patch
---

Fixed [#11566](https://github.com/biomejs/biome/issues/11566): [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer reports a `namespace` declared inside `declare global` or inside an external module declaration. Both positions are documented as always ignored, and the rule offered a safe fix, so `biome check --write` renamed the declaration:

```ts
export {}
declare global {
    // no longer renamed to `Jsx`
    namespace JSX {}
}
```
