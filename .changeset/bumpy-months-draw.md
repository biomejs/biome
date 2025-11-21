---
"@biomejs/biome": patch
---

Fixed [#7390](https://github.com/biomejs/biome/issues/7390), where Biome couldn't apply the correct configuration passed via `--config-path`.

Now if you have multiple **root** configuration files, running any command with `--config-path` will apply the chose configuration file.
