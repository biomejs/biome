---
"@biomejs/biome": patch
---

Fixed the language detection of files inside the global settings directories of Zed, VS Code, and Cursor. Biome still parses `.json` files there as JSON with comments and trailing commas, and now detects every other file there, such as `script.ts` or `notes.md`, by its own extension instead of treating it as JSON.
