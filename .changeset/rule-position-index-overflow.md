---
"@biomejs/biome": patch
---

Fixed an issue where rule indices past 255 in a lint group wrapped around due to a u8 cast, causing rules such as `useWhileLoopCondition` to map to the wrong rule when enabled in configuration.
