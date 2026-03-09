---
"@biomejs/biome": minor
---

Added the `sortBareImports` option to [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/),
which allows bare imports to be sorted within other imports when set to `false`.

```json
{
  "assist": {
    "actions": {
      "source": {
        "organizeImports": {
          "level": "on",
          "options": { "sortBareImports": true }
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
