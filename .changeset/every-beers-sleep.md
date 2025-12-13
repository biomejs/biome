---
"@biomejs/biome": minor
---

Added new CLI options to the commands `lsp-proxy` and `start` that allow to control the Biome file watcher.

#### `--watcher-kind`

Controls how the Biome file watcher should behave. By default, Biome chooses the best watcher strategy for the
current OS, however sometimes this could result in some issues, such as folders locked.

The option accepts the current values:
- `recommended`: the default option, which chooses the best watcher for the current platform.
- `polling`: uses the polling strategy.
- `none`: it doesn't enable the watcher. When the watcher is disabled, changes to files aren't recorded anymore by Biome. This might have
  repercussions on some lint rules that might rely on updated types or updated paths.

The environment variable `BIOME_WATCHER_KIND` can be used as alias.

#### `--watcher-polling-interval`

The polling interval in milliseconds. This is only applicable when using the `polling` watcher. It defaults to `2000` milliseconds.

The environment variable `BIOME_WATCHER_POLLING_INTERVAL` can be used as alias.
