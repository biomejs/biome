---
"@biomejs/biome": patch
---

[`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies) can now validate custom hooks whose dependency arrays come before their callbacks.

Previously, a logical error caused the rule to be unable to detect dependency arrays placed before hook callbacks, producing spurious errors that blocked further diagnostics.
```json
{
  "linter": {
    "rules": {
      "correctness": {
        "useExhaustiveDependencies": {
          "level": "error",
          "options": {
            "hooks": [
              {
                "name": "doSomething",
                "closureIndex": 2,
                "dependenciesIndex": 0
              }
            ]
          }
        }
      }
    }
  }
}
```

```js
function component() {
  let thing = 5;
  // The rule will now correctly flag `thing` as missing instead of complaining about
  // missing dependency arrays
  doSomething([], "blah", () => {console.log(thing)})
}
```

The rule documentation & diagnostic messages have also been reworked for improved clarity.
