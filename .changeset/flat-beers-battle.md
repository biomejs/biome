---
"@biomejs/biome": minor
---

Added the `ignoreBareImports` option to [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/),
which allows bare imports to be sorted within other imports when set to `false`.

```json
{
  "assist": {
    "actions": {
      "source": {
        "organizeImports": {
          "level": "on",
          "options": { "ignoreBareImports": false }
        }
      }
    }
  }
}
```

```diff
- import "b";
  import "a";
+ import "b";
  import { A } from "a";
+ import "./file";
  import { Local } from "./file";
- import "./file";
```

Note that bare imports are never merged with other imports.
