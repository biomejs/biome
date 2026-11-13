---
"@biomejs/biome": minor
---

Added the `assertFunctionNames` option to the [`useExpect`](https://biomejs.dev/linter/rules/use-expect/) rule. Closes [#9218](https://github.com/biomejs/biome/issues/9218).

The option takes a list of additional function names that count as assertions. This is useful for helpers that wrap `expect()`, and for assertion libraries that the rule doesn't know.

```json
{
    "linter": {
        "rules": {
            "suspicious": {
                "useExpect": {
                    "level": "error",
                    "options": {
                        "assertFunctionNames": ["expectSaga", "utils.assertValid"]
                    }
                }
            }
        }
    }
}
```

With this configuration, the following tests are valid:

```js
test("runs the saga", () => {
    expectSaga(saga).put({ type: "DONE" }).run();
});

test("is valid", () => {
    utils.assertValid(compute());
});
```
