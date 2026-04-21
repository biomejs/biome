---
"@biomejs/biome": minor
---

Added a `:BARE:` predefined group matcher to the [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) assist action. The new matcher selects bare (side-effect) imports such as `import "polyfill"` and can be combined with `sortBareImports: true` and the `groups` option to express orderings such as "place all side-effect imports at the end of the import list".

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
                            ["**", "!:BARE:"],
                            ":BLANK_LINE:",
                            [":BARE:"]
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
