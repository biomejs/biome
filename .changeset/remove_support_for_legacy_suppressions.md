---
cli: major
---

# Remove support for legacy suppressions

Biome used to support "legacy suppressions" that looked like this:

```js
// biome-ignore lint(style/useWhile): reason
```

This format is no longer supported.
