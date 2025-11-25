---
"@biomejs/biome": patch
---

Addressed [#7538](https://github.com/biomejs/biome/issues/7538). Reduced the
volume of logging from the LSP server. Biome has been creating very large
(100GB+) log files. It was logging the entire text document twice on almost
every keystroke, and that much has been fixed.

If you are affected by this issue, you can run `biome clean` to delete logs,
otherwise they are cleaned up weekly by default. If you still find Biome is
creating very large log files after this change, please comment on #7538.
