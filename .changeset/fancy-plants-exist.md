---
"@biomejs/biome": patch
---

Fixed [#748](https://github.com/biomejs/biome-vscode/issues/748), where Biome Language Server didn't show the unsafe fixes when requesting the quick fixes. Now all LSP editors will show also opt-in, unsafe fixes.
