---
"@biomejs/biome": patch
---

The Tailwind parser now recovers from a malformed class at the next space, so the classes after it are still parsed.
