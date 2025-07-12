---
"@biomejs/biome": minor
---

fix(cli): rage command now respects --config-path and BIOME_CONFIG_PATH

The rage command now loads the configuration file specified by the --config-path CLI option or the BIOME_CONFIG_PATH environment variable, if set. Previously, it always loaded the default biome.json file. This closes #6686.
