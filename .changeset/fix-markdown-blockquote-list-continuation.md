---
"@biomejs/biome": patch
---

Fixed markdown parsing for lists inside blockquotes separated by blank blockquote continuation lines. Biome now keeps same-marker list items in one loose list while still splitting different marker or cross-type lists.
