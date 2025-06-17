---
"@biomejs/biome": minor
---

Add an `ignoreRestSiblings` option into [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables).

When this option is set to `false`, the rule will **not** ignore variables that created using the rest pattern:

```json
{
  "linter": {
    "rules": {
      "correctness": {
        "noUnusedVariables": {
          "level": "error",
          "options": {
            "ignoreRestSiblings": false
          }
        }
      }
    }
  }
}
```

```js
const { lorem, ...test } = bar; // the variable "test" will trigger the rule
console.log(lorem)
```
