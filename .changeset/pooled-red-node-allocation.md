---
"@biomejs/biome": patch
---

Improved the performance of linting with many rules enabled, such as Biome's recommended rules. In exchange, Biome can use more memory, because each thread now keeps some freed memory for reuse.
