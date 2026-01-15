---
"@biomejs/biome": patch
---

Fixed [#7982](https://github.com/biomejs/biome/issues/7982):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly handles callback expressions with type
assertions.

```tsx
const callback = useCallback(
  (() => {
    return count * 2;
  }) as Function,
  [count], // count is now correctly detected
);
```
