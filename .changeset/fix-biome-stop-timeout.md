---
"@biomejs/biome": patch
---

Fixed `biome stop` waiting 15 seconds and then failing with "the request to the remote workspace timed out", even though the Biome daemon had stopped. The command now returns as soon as the daemon shuts down.
