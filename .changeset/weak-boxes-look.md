---
"@biomejs/biome": patch
---

Fixed [#9217](https://github.com/biomejs/biome/issues/9217) and [biomejs/biome-vscode#959](https://github.com/biomejs/biome-vscode/issues/959), where the Biome language server didn't correctly resolve the editor setting `configurationPath` when the provided value is a relative path.
