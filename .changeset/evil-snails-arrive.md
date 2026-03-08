---
"@biomejs/biome": patch
"@biomejs/cli-darwin-arm64": patch
"@biomejs/cli-darwin-x64": patch
"@biomejs/cli-linux-arm64": patch
"@biomejs/cli-linux-arm64-musl": patch
"@biomejs/cli-linux-x64": patch
"@biomejs/cli-linux-x64-musl": patch
"@biomejs/cli-win32-arm64": patch
"@biomejs/cli-win32-x64": patch
---

Fixed [biomejs/biome-vscode#959](https://github.com/biomejs/biome-vscode/issues/959): LSP now correctly resolves project directory when `configurationPath` points to a configuration file outside the workspace.
