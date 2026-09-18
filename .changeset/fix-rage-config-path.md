---
"@biomejs/biome": patch
---

Fixed [#6686](https://github.com/biomejs/biome/issues/6686): the `rage` command now respects the `--config-path` option and the `BIOME_CONFIG_PATH` environment variable when loading the Biome configuration. Previously it always used the default configuration resolution and reported the configuration as `Not set` when no `biome.json` existed in the working directory.
