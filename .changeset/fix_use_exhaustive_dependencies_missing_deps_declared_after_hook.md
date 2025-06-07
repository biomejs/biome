---
"@biomejs/biome": patch
---

Fixes useExhaustiveDependencies missing dependencies being defined after the hook itself failure.

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
