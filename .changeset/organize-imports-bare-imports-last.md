---
"@biomejs/biome": minor
---

Added a `bareImports` option to the [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) assist action. Setting it to `"last"` groups side-effect (bare) imports at the end of the import list instead of preserving their position as chunk boundaries. Defaults to `"preserve"` for backwards compatibility.

For example, with `"bareImports": "last"`:

```json
{
    "assist": {
        "actions": {
            "source": {
                "organizeImports": {
                    "level": "on",
                    "options": { "bareImports": "last" }
                }
            }
        }
    }
}
```

The following code...

```ts
import "./polyfills";
import { Button } from "@/components/Button";
import "./styles.css";
import { render } from "react-dom";
```

...is organized as:

```ts
import { render } from "react-dom";
import { Button } from "@/components/Button";

import "./polyfills";
import "./styles.css";
```

Duplicate bare imports from the same source are merged.
