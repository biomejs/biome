---
"@biomejs/biome": patch
---

Fixed [#4248](https://github.com/biomejs/biome/issues/4248):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now correctly handles function props passed as
callbacks.

```tsx
const data = React.useMemo(getData, [getData]); // getData is now correctly recognized as needed
```
