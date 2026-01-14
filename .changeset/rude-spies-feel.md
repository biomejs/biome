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

Fixed [#3512](https://github.com/biomejs/biome/issues/3512):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly handles nested destructuring patterns
from hook results.

```tsx
const [[x, y], setXY] = useState([1, 2]);
useEffect(() => {
  console.log(x, y);
}, [x, y]); // x and y are now correctly recognized as unstable
```

Fixed [#3685](https://github.com/biomejs/biome/issues/3685):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly handles transparent expression
wrappers like non-null assertions and type assertions in dependency comparisons.

```tsx
useMemo(() => Boolean(myObj!.x), [myObj!.x]); // No longer reports incorrect diagnostics
useMemo(() => myObj!.x?.y === true, [myObj!.x?.y]); // Now correctly matches dependencies
```

Fixed [#5914](https://github.com/biomejs/biome/issues/5914):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly handles variables declared in the same
statement.

```tsx
const varA = Math.random(),
  varB = useMemo(() => varA, [varA]); // varA is now correctly recognized as needed
```

Fixed [#8427](https://github.com/biomejs/biome/issues/8427):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly resolves variable references to detect
captured dependencies.

```tsx
const fe = fetchEntity;
useEffect(() => {
  fe(id);
}, [id, fe]); // fe is now correctly detected as needed
```

Fixed [#8484](https://github.com/biomejs/biome/issues/8484):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly handles member access on stable hook
results.

```tsx
const stableObj = useStable();
useMemo(() => {
  return stableObj.stableValue; // stableObj.stableValue is now correctly recognized as stable
}, []);
```

Fixed [#3080](https://github.com/biomejs/biome/issues/3080):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly analyzes captures within referenced
functions passed to hooks.

```tsx
function myEffect() {
  console.log(foo, bar);
}
useEffect(myEffect, [foo, bar]); // foo and bar are now correctly detected
```

Fixed [#4248](https://github.com/biomejs/biome/issues/4248):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now correctly handles function props passed as
callbacks.

```tsx
const data = React.useMemo(getData, [getData]); // getData is now correctly recognized as needed
```
