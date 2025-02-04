---
"@biomejs/biome": minor
---

You can now enable lint rules using the default severity suggested by Biome using the new variant `"on"`, when enabling a rule.

For example, the default severity of the rule `style.noVar` is `error`, so you would use `"on"`, and  then linting a code that uses `var`, will result in an error:

```json
{
  "linter": {
    "recommended": false,
    "rules": {
      "style": {
        "noVar": "on"
      }
    }
  }
}
```

```js
// main.js
var name = "tobias"
```

The command `biome lint main.js` will result in an error due to the default severity assigned to `noVar`.

Refer to the documentation page of each rule to know their suggested diagnostic severity, or use the command `biome explain <RULE_NAME>`:

```shell
biome explain noVar
```
