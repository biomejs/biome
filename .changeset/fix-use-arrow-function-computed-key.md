---
"@biomejs/biome": patch
---

Fixed [#13128](https://github.com/biomejs/biome/issues/13128): `useArrowFunction` now ignores functions where enclosed class or object member computed keys reference `this`, `new.target`, or `arguments`.
