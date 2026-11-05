---
"@biomejs/biome": patch
---

Improved the performance of linting. Syntax nodes that rules create while inspecting code now reuse memory freed by earlier ones instead of asking the system allocator each time.
