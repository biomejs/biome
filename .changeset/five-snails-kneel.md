---
"@biomejs/biome": patch
---

Fixed the Tailwind class parser rejecting CSS variable shorthands whose fallback contains parentheses. Classes like `bg-(--a,var(--b))`, `bg-(--a,calc(1px))`, and `after:bg-(--drawer-bleed-background,var(--color-popover))` now parse correctly, including in variants (`max-(--a,var(--b)):flex`) and modifiers (`bg-red-500/(--a,var(--b))`).
