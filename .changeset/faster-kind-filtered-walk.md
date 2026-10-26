---
"@biomejs/biome": patch
---

Improved the performance of linting. Each lint pass now spends less time stepping over syntax nodes that no enabled rule looks at.
