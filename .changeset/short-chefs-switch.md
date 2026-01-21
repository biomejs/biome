---
"@biomejs/biome": patch
---

Fixed [#3080](https://github.com/biomejs/biome/issues/3080):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly analyzes captures within referenced
functions passed to hooks.

```tsx
function myEffect() {
  console.log(foo, bar);
}
useEffect(myEffect, [foo, bar]); // foo and bar are now correctly detected
```
