---
"@biomejs/biome": patch
---

Restricted access to the Unix daemon socket to the user running Biome. The socket now lives in a `biome-daemon` directory inside Biome's cache directory that only this user can access, and the socket itself has mode `0600`.
