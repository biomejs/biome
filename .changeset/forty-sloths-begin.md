---
"@biomejs/biome": patch
---

Fixed [#3685](https://github.com/biomejs/biome/issues/3685):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly handles transparent expression
wrappers like non-null assertions and type assertions in dependency comparisons.

```tsx
useMemo(() => Boolean(myObj!.x), [myObj!.x]); // No longer reports incorrect diagnostics
useMemo(() => myObj!.x?.y === true, [myObj!.x?.y]); // Now correctly matches dependencies
```
