---
"@biomejs/js-api": minor
---

Added `spanInBytesToSpanInCodeUnits` utility function to convert UTF-8 byte offset spans returned by Biome into JavaScript string (UTF-16 code unit) offsets. This fixes [`#4035`](https://github.com/biomejs/biome/issues/4035): diagnostic spans are now usable with `String.prototype.slice()` for strings containing non-ASCII characters.
