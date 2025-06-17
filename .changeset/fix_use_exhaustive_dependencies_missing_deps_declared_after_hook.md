---
"@biomejs/biome": patch
---

Fixes [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) missing dependencies being defined after the hook itself failure.

Example:

```jsx
import { useState, useEffect } from 'react';

function MyComponent() {
  useEffect(() => {
    console.log(a);
  }, []);

  let a = 1;
}
```
