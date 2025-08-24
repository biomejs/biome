---
"@biomejs/biome": patch
---

Added a test for `lint/complexity/noExtraBooleanCast` covering [#7225](https://github.com/biomejs/biome/issues/7225). The test verifies that the fixer preserves parentheses when removing `Boolean` inside a negation.

```js
// Before
!Boolean(b0 && b1)
// After
!(b0 && b1)
```
This ensures operator precedence is preserved.
