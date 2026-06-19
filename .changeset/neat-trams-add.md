---
"@biomejs/biome": patch
---

Fixed [#10658](https://github.com/biomejs/biome/issues/10658). The issue was caused by the "Go-to definition" editor feature, which was enabled by default. The feature is now **disabled by default**. To work, the feature triggers the scanner to build the module graph. This caused memory leak issues in cases where Biome starts in the home directory to modify files.

If you relied on this new feature, you must now turn on using the [editor settings] of the extension e.g. [Zed](https://biomejs.dev/reference/zed/#goto_definition) and [VSCode](https://biomejs.dev/reference/vscode/#biomegotodefinition).
