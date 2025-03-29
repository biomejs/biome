---
"@biomejs/biome": minor
---

Added an **unsafe** fix to the rule [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies).

For example, this violation will provide the following code fix:

```js
import { useEffect } from "react";

function MyComponent() {
  let a = 1;
  useEffect(() => {}, [a]);
}
```

```
  × This hook specifies more dependencies than necessary: a

    3 │ function MyComponent() {
    4 │   let a = 1;
  > 5 │   useEffect(() => {}, [a]);
      │   ^^^^^^^^^
    6 │ }
    7 │

  i This dependency can be removed from the list.

    3 │ function MyComponent() {
    4 │   let a = 1;
  > 5 │   useEffect(() => {}, [a]);
      │                        ^
    6 │ }
    7 │

  i Unsafe fix: Remove the extra dependencies from the list.

    5 │ ··useEffect(()·=>·{},·[a]);
      │                        -
```
