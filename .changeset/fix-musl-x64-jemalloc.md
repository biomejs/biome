---
"@biomejs/biome": patch
---

Fixed the `linux-x64-musl` binary not using jemalloc, which it already ships: it fell back to musl's allocator, and `biome check` on several threads could take minutes instead of seconds, for example in Alpine-based CI images.
