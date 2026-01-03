---
"@biomejs/biome": patch
---

Added the `checkForEach` option to the `useIterableCallbackReturn` rule.

When `checkForEach` is `true`, the rule reports when `forEach` callbacks return a value. When `false` (the default), `forEach` callbacks are not checked.

This aligns with ESLint's `array-callback-return` rule behavior where `forEach` checking is opt-in.

**Breaking change:** Previously, `forEach` was always checked. Users who want the previous behavior should set `checkForEach: true` in their configuration.
