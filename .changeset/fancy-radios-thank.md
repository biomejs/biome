---
"@biomejs/biome": major
---

The lint rule [`noRestrictedGlobals`](https://biomejs.dev/linter/rules/no-restricted-globals/) now supports customizing message for each global name.

For example, the following configuration:

```json
{
  "options": {
    "deniedGlobals": {
      "$": "jQuery is not allowed. Use native DOM manipulation instead."
    }
  }
}
```

emits a diagnostic:

```
index.js:1:13 lint/style/noRestrictedGlobals ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ⚠ Do not use the global variable $.

  > 1 │ console.log($)
      │             ^
    2 │

  ℹ jQuery is not allowed. Use native DOM manipulation instead.
```

Breaking Change: The option `deniedGlobals` is now a record instead of an array. Run `biome migrate` to migrate the configuration automatically.
