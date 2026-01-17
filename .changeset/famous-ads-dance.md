---
"@biomejs/biome": patch
---

Fixed [#3512](https://github.com/biomejs/biome/issues/3512):
[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) now properly handles nested destructuring patterns
from hook results.

```tsx
const [[x, y], setXY] = useState([1, 2]);
useEffect(() => {
  console.log(x, y);
}, [x, y]); // x and y are now correctly recognized as unstable
```
