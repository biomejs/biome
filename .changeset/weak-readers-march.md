---
"@biomejs/biome": patch
---

Fixed a bug where Biome didn't throw any error when `vcs.useIgnoreFile` is set to `true`, and there wasn't any ignore file read. Now Biome correctly throws an error if no ignore files are found.
