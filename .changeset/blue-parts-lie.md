---
"@biomejs/biome": patch
---

Fixed [#8233](https://github.com/biomejs/biome/issues/8233), where Biome CLI in
stdin mode didn't work correctly when handling files in projects with nested
configurations. For example, with the following structure,
`--stdin-file-path=subdirectory/...` would not use the nested configuration in
`subdirectory/biome.json`:

```
├── biome.json
└── subdirectory
    ├── biome.json
    └── lib.js
```

```shell
biome format --write --stdin-file-path=subdirectory/lib.js < subdirectory/lib.js
```

Now, the nested configuration is correctly picked up and applied.

In addition, Biome now shows a warning if `--stdin-file-path` is provided but
that path is ignored and therefore not formatted or fixed.
