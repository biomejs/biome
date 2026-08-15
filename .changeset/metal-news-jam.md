---
"@biomejs/biome": patch
---

Fixed [#11335](https://github.com/biomejs/biome/issues/11335): [`noComponentHookFactories`](https://biomejs.dev/linter/rules/no-component-hook-factories/) now reports a `use`-prefixed variable only when a function is assigned to it directly.

```js
function factory() {
  const useColors = true; // no longer reported
  const useStore = createStore({ count: 0 }); // no longer reported
  const useData = () => useState(null); // still reported
  return useColors;
}
```
