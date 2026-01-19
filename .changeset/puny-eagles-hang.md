---
"@biomejs/biome": patch
---

Fixed [#8427](https://github.com/biomejs/biome/issues/8427):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly resolves variable references to detect
captured dependencies.

```tsx
const fe = fetchEntity;
useEffect(() => {
  fe(id);
}, [id, fe]); // fe is now correctly detected as needed
```
