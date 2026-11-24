---
"@biomejs/biome": patch
---

Fixed [#13386](https://github.com/biomejs/biome/issues/13386): lists nested inside block quotes no longer look past unquoted newlines or blank lines for outer list markers, avoiding spurious `Expected a list item` parse diagnostics when a quoted list item ends with a fenced code block followed by a sibling outer item.
