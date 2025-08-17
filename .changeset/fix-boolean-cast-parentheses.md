---
"@biomejs/biome": patch
---

Fixed [#7225](https://github.com/biomejs/biome/issues/7225): The noExtraBooleanCast rule now preserves parentheses when removing Boolean calls inside negations.

```js
// Before
!Boolean(b0 && b1)
// After  
!(b0 && b1)  // instead of !b0 && b1
```
