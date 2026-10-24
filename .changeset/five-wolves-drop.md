---
"@biomejs/biome": patch
---

Fixed [biomejs/biome-vscode#923](https://github.com/biomejs/biome-vscode/issues/923): when a file open in the editor changed on disk, the language server replaced the editor's text with the text on disk. Biome now keeps the editor's text of open files until they are closed, so diagnostics, code actions and formatting stay at the right positions.
