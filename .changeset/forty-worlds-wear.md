---
"@biomejs/biome": patch
---

Fixed `scanProject` in the WebAssembly packages, which scanned files inside folders excluded by `files.includes`, for example `dist/minified.js` with `"!**/dist"`. Biome now skips these files, like the CLI does.
