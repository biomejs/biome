---
"@biomejs/biome": patch
---

Fix a parsing error when a `JsxElementName` is `JsxMemberExpression`, and a `JsLogicalExpreesion` before it without a semicolon.

The following case will now not throw error:

```jsx
import React from 'react';

let b = 0;

function A() {
    const a = b > 0 && b < 1

    return (
        <React.Fragment>
          {a}
        </React.Fragment>
    )
}
```

