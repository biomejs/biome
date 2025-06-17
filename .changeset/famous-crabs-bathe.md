---
"@biomejs/biome": minor
---

Added the rule [useUniqueElementIds](https://biomejs.dev/linter/rules/use-unique-element-ids/).
This rule disallows the use of static IDs in React components. It encourages to generate unique IDs for accessibility purposes using [`useId`](https://react.dev/reference/react/useId).

The following code is now reported as invalid:

```jsx
function App() {
  return <div id="static-id" />;
}
```

The following code is now reported as valid:

```jsx
import { useId } from "react";
function App() {
  const id = useId();
  return <div id={id} />;
}
```
