---
"@biomejs/biome": patch
---

Fixed `lint/a11y/useAnchorContent` false positive for `<a>` elements used as render prop values (e.g. `render={<a href="..." />}`), a pattern where the receiving component renders its children inside the anchor element.
