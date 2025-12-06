---
"@biomejs/biome": patch
---

Revamped the logging options for all Biome commands. Now the commands `format`, `lint`, `check`, `ci`, `search`, `lsp-proxy` and `start` accept the following CLI options.

Some options might have been present before, but they were inconsistent. Plus, all new options have an environment variable as aliases.

#### `--log-file`

Optional path/file to redirect log messages to. This option is applicable only to the CLI. If omitted, logs are printed to stdout.

Environment variable alias: `BIOME_LOG_FILE`

#### `--log-prefix-name`

Allows changing the prefix applied to the file name of the logs. This option is applicable only to the daemon.

Environment variable alias: `BIOME_LOG_PREFIX_NAME`

#### `--log-path`

Allows changing the folder where logs are stored. This option is applicable only to the daemon.

Environment variable alias: `BIOME_LOG_PATH`

#### `--log-level`

The level of logging. In order, from the most verbose to the least verbose: `debug`, `info`, `warn`, `error`

The value `none` won't show any logging.

Environment variable alias: `BIOME_LOG_LEVEL`

#### `--log-kind`

What the log should look like.

Environment variable alias: `BIOME_LOG_KIND`

#### Reduce dumping of LSP logs

When you use a Biome editor extension, Biome's Daemon dumps its logs using the `debug` level. If you want to reduce
the quantity of these logs, you can now customize it:

```shell
BIOME_LOG_LEVEL=info biome lsp-proxy
```
