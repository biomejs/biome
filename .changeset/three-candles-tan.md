---
"@biomejs/biome": patch
---

Fixed a regression where Biome LSP could misread editor settings sent through `workspace/didChangeConfiguration` when the payload was wrapped in a top-level `biome` key. This caused `requireConfiguration` and related settings to be ignored in some editors.
