---
"@biomejs/biome": patch
---

Fixed [#6838](https://github.com/biomejs/biome/issues/6838), where the Biome File Watcher incorrectly watched and stored ignored files, causing possible memory leaks when those files were dynamically created (e.g. built files).
