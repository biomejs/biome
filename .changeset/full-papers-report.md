---
"@biomejs/biome": patch
---

Fixed parsing of bare `if` identifiers in CSS `if()` branch values, such as `if(style(--enabled: true): if; else: serif)`.
