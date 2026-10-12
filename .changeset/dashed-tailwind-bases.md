---
"@biomejs/biome": patch
---

Fixed the Tailwind CSS parser splitting dashed utility names. Classes such as `drop-shadow!`, `flex-grow-0`, and `max-w-screen-lg` now keep `drop-shadow`, `flex-grow`, and `max-w-screen` as their utility names.
