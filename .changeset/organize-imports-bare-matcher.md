---
"@biomejs/biome": minor
---

Added an optional `kind` field to the import matchers used by the [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) assist action.
The new field selects imports by their syntactic kind and currently supports `bare` (matching side-effect imports such as `import "polyfill"`) with optional `!` negation (`!bare`).
The matcher composes with the existing `type` and `source` fields, so users can express patterns such as "only bare imports that import style files" (`{ "kind": "bare", "source": ":STYLE:" }`).

For example, with the following configuration:

```json
{
  "assist": {
    "actions": {
      "source": {
        "organizeImports": {
          "level": "on",
          "options": {
            "groups": [
              { "kind": "!bare" },
              ":BLANK_LINE:",
              { "kind": "bare", "source": ":STYLE:" }
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
import "./b.css";
import d from "./d.js";
import "./a.css";
```

...is organized as:

```ts
import d from "./d.js";

import "./a.css";
import "./b.css";
```
