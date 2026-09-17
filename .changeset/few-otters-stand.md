---
"@biomejs/biome": minor
---

Configuration files can now recursively extend local files and npm package configurations up to ten levels. Every repeated reference participates in the merge, while resolving multiple versions of one extended package produces an error.
