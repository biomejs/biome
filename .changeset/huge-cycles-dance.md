---
"@biomejs/biome": minor
---

Implements [#1984](https://github.com/biomejs/biome/issues/1984). Updated [`useHookAtTopLevel`](https://biomejs.dev/linter/rules/use-hook-at-top-level/) to better catch invalid hook usage.

This rule is now capable of finding invalid hook usage in more locations. A diagnostic will now be generated if:
- A hook is used at the module level (top of the file, outside any function).
- A hook is used within a function or method which is not a hook or component, unless it is a function expression (such as arrow functions commonly used in tests).

**Invalid:**

```js
// Invalid: hooks cannot be called at the module level.
useHook();
```

```js
// Invalid: hooks must be called from another hook or component.
function notAHook() {
  useHook();
}
```

**Valid:**

```js
// Valid: hooks may be called from function expressions, such as in tests.
test("my hook", () => {
  renderHook(() => useHook());

  renderHook(function() {
    return useHook();
  });
});
```
