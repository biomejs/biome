---
"@biomejs/biome": patch
---

Fixed module resolution in long-running workspaces so imports reflect package manifest and TypeScript path-mapping changes without requiring the importing file to be edited.
