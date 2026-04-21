---
"@biomejs/biome": minor
---

Added the `:STYLE:` group matcher for [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) that matches style imports.

For example, the following configuration...

```json
{
  "assist": {
    "actions": {
      "source": {
        "organizeImports": {
          "level": "on",
          "options": {
            "groups": ["**", "!:STYLE:"],
            "sortBareImports": true
          }
        }
      }
    }
  }
}
```

...places style imports last:

```diff
- import "./style.css"
  import A from "./a.js"
+ import "./style.css"
```
