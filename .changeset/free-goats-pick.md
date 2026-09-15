---
"@biomejs/biome": patch
---

Fixed dashed utility base names in the Tailwind parser, including `border-bs`, `font-features`, and `scrollbar-thumb`. Classes such as `min-inline-[12rem]` now preserve the complete base name and parse the arbitrary value separately.
