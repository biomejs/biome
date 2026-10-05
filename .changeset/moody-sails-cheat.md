---
"@biomejs/biome": patch
---

Fixed the Tailwind class parser producing an invalid syntax tree for parenthesized and unary expressions inside arbitrary values. Classes like `w-[calc(1px+(2px))]`, `w-[calc((100%_-_2rem)/3)]`, and `w-[calc(1px*-(2px))]` now parse correctly.
