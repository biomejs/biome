---
"@biomejs/biome": patch
---

Fixed an issue with the `files.maxSize` setting. Previously the setting would always be looked up in the root settings, even in monorepos where a closer `biome.json` is available. It now correctly uses the nearest configuration.
