---
"@biomejs/biome": patch
---

Fix `--changed` and `--staged` flags throwing "No such file or directory" error when a file has been deleted or renamed in the working directory. The CLI now filters out files that no longer exist before processing.
