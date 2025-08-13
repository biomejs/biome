---
"@biomejs/biome": minor
---

Added support for tracking stable results in user-provided React hooks that return objects to [`useExhaustiveDependencies`](https://biomejs.dev/linter/rules/use-exhaustive-dependencies/) to compliment existing support for array return values. For example:

```json5
// biome.json
{
  // rule options
  "useExhaustiveDependencies": {
      "level": "error",
      "options": {
          "hooks": [{
              "name": "useCustomHook",
              "stableResult": [
                  "setMyState"
              ]
          }]
      }
  }
}
```

This will allow the following to be validated:

```js
const {myState, setMyState} = useCustomHook();
const toggleMyState = useCallback(() => {
  setMyState(!myState);
}, [myState]); // Only `myState` needs to be specified here.
```
