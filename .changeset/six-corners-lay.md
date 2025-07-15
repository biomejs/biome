---
"@biomejs/biome": minor
---

Added new functions:
- `fileExists`: returns whether the input file exists in the workspace.
- `isPathIgnored`: returns whether the input path is ignored.
- `updateModuleGraph`: updates the internal module graph of the input path.
- `getModuleGraph`: it returns a serialized version of the internal module graph.
