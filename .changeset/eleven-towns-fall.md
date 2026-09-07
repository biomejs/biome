---
"@biomejs/biome": minor
---

Added `biome inspect config` to show resolved configuration values and their origins, including matching overrides.

Configuration files can also extend other configurations up to ten levels deep. Repeated references to the same resolved file are ignored with an information diagnostic, while resolving multiple versions of one extended package produces an error.
