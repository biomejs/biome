---
"@biomejs/biome": patch
---

Fixed the Markdown parser's quoted hard-line-break handling so blank-line checks no longer consume quote prefixes while probing. This preserves correct continuation parsing for quoted content after hard breaks.
