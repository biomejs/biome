---
"@biomejs/biome": minor
---

Added a `kind` field to the `ImportMatcher` used by the [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) assist action. The new field selects imports by their syntactic kind and currently supports `bare` (matching side-effect imports such as `import "polyfill"`) with optional `!` negation (`!bare`). The matcher composes with the existing `type` and `source` fields, so users can express patterns such as "only bare imports that import a CSS file" (`{ "kind": "bare", "source": "**/*.css" }`).

For example, with the following configuration:

```json
{
    "assist": {
        "actions": {
            "source": {
                "organizeImports": {
                    "level": "on",
                    "options": {
                        "sortBareImports": true,
                        "groups": [
                            { "kind": "!bare" },
                            ":BLANK_LINE:",
                            { "kind": "bare" }
                        ]
                    }
                }
            }
        }
    }
}
```

...the following code:

```ts
import "./register-my-component";
import { render } from "react-dom";
import "./polyfill";
import { Button } from "@/components/Button";
```

...is organized as:

```ts
import { render } from "react-dom";
import { Button } from "@/components/Button";

import "./polyfill";
import "./register-my-component";
```
