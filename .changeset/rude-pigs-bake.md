---
"@biomejs/biome": minor
---

Add an `ignoreRestSiblings` option into [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables).

When this option is set to `true`, the rule will ignore variables that created using the rest pattern:

```json
{
  "linter": {
    "rules": {
      "correctness": {
        "noUnusedVariables": {
          "level": "error",
          "options": {
            "ignoreRestSiblings": true
          }
        }
      }
    }
  }
}
```

```js
const { lorem, ...test } = bar; // the variable "test" won't trigger the rule
console.log(lorem)
```
