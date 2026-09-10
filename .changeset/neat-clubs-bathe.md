---
"@biomejs/biome": minor
---

Added the new command `biome inspect file`, which returns developer information for the given file.

```shell
biome inspect file path/to/file.ts
```

By default, the command returns the CST of the file. The command accepts the following arguments:
- `--ast`, which returns the AST of a file.
- `--ir`, which returns the formatting IR of a file.
- `--semantic`, which returns the semantic model information of a file, if available.
