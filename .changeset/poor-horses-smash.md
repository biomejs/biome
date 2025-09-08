---
"@biomejs/biome": patch
---

Fixed an issue (#6393) where the [useHookAtTopLevel](https://biomejs.dev/linter/rules/use-hook-at-top-level/) rule reported excessive diagnostics for nested hook calls.

The rule now reports only the offending top-level call site, not sub-hooks of composite hooks.

```js
// Before: reported twice (useFoo and useBar).
function useFoo() { return useBar(); }
function Component() {
  if (cond) useFoo();
}
// After: reported once at the call to useFoo().
```
