---
"@biomejs/biome": patch
---

Fixed [#8240](https://github.com/biomejs/biome/issues/8240): the Biome daemon no longer gets stuck at 100% CPU after a burst of file changes, such as a git branch switch. Before, the file watcher could deadlock, after which Biome stopped seeing changes made outside the editor until it was restarted.
