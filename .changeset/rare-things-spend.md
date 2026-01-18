---
"@biomejs/biome": patch
---

Fixed [#8484](https://github.com/biomejs/biome/issues/8484):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly handles member access on stable hook
results.

```tsx
const stableObj = useStable();
useMemo(() => {
  return stableObj.stableValue; // stableObj.stableValue is now correctly recognized as stable
}, []);
```
