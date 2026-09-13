---
"@biomejs/biome": patch
---

Fixed validation of `readonly` and `accessor` modifiers: combining them in either order now reports that they cannot be used together.
