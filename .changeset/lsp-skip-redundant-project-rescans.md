---
"@biomejs/biome": patch
---

Fixed [#10605](https://github.com/biomejs/biome/issues/10605): the LSP daemon no longer reruns a full project scan when a `workspace/didChangeConfiguration` notification arrives and the Biome configuration has not changed. Editors send this notification for any settings change, and every notification previously forced a full project rescan.
