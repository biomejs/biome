---
"@biomejs/biome": patch
---

Fixed a bug where the Biome Language Server would apply an unsafe fix when using the code action `quickfix.biome`.

Now Biome no longer applies an unsafe code fix when using the code action `quickfix.biome`.
