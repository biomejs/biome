---
"@biomejs/biome": patch
---

Fixed [#10605](https://github.com/biomejs/biome/issues/10605): the LSP server no longer runs a full project scan when the configuration is reloaded but hasn't changed. Previously, any editor settings change triggered a full rescan.
