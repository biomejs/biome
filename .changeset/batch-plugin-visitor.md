---
"@biomejs/biome": patch
---

Improved plugin performance by batching all plugins into a single syntax visitor with a kind-to-plugin lookup map, reducing per-node dispatch overhead from O(N) to O(1) where N is the number of plugins.
