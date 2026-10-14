---
"@biomejs/biome": patch
---

Fixed errors that the Biome language server sent to the editor with an empty message. For example, when a nested configuration file sets `"root": true`, the editor logs now show "Found a nested root configuration, but there's already a root configuration." instead of an empty line.
