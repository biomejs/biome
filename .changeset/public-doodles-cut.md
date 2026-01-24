---
"@biomejs/biome": patch
---

Fixed [#5914](https://github.com/biomejs/biome/issues/5914):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly handles variables declared in the same
statement.

```tsx
const varA = Math.random(),
  varB = useMemo(() => varA, [varA]); // varA is now correctly recognized as needed
```
