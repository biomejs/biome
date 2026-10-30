---
"@biomejs/biome": patch
---

Fixed [#12625](https://github.com/biomejs/biome/issues/12625): [`noComponentHookFactories`](https://biomejs.dev/linter/rules/no-component-hook-factories/) no longer reports a capitalised object property inside a function unless its function renders JSX.

```ts
function methods(player) {
  return { Next: () => player.next() };
}
```
