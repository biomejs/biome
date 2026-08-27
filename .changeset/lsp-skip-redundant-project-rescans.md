---
"@biomejs/biome": patch
---

Fixed redundant full project rescans in the LSP daemon. Editors send `workspace/didChangeConfiguration` for any settings change, and every notification reloaded the configuration and forced a full project scan even when the Biome configuration was unchanged. The session now fingerprints the loaded configuration and skips the settings update and the rescan when nothing changed. Concurrent scan requests for the same project are also coalesced into at most one running scan plus one queued follow-up, instead of piling up.
