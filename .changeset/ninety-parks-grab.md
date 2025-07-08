---
"@biomejs/biome": patch
---

Fixed [#6566](https://github.com/biomejs/biome/issues/6566): Biome no longer
errors when using the option `--files-ignore-unknown=true` in `stdin` mode.

Biome has also become less strict when using `--stdin-file-path` in `stdin`
mode. It will no longer error if the file path doesn't contain an extension, but
instead it will return the original content.
