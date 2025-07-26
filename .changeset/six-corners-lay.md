---
"@biomejs/biome": minor
"@biomejs/wasm-bundler": minor
"@biomejs/wasm-nodejs": minor
"@biomejs/wasm-web": minor
---

Added new functions to the `@biomejs/wasm-*` packages:
- `fileExists`: returns whether the input file exists in the workspace.
- `isPathIgnored`: returns whether the input path is ignored.
- `updateModuleGraph`: updates the internal module graph of the input path.
- `getModuleGraph`: it returns a serialized version of the internal module graph.
- `scanProjectFolder`: scans the files and directories in the project to build the internal module graph.
