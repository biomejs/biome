---
"@biomejs/biome": patch
---

Fixed [#11566](https://github.com/biomejs/biome/issues/11566): [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) no longer reports namespaces declared inside a `declare global` block. The rule already ignored every other declaration in that position, and its safe fix renamed `namespace JSX` to `namespace Jsx`.

Biome no longer reports the namespace below:

```ts
declare global {
    namespace JSX {
        interface IntrinsicElements {}
    }
}
```
