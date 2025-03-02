---
"@biomejs/biome": major
---

Removed `--config-path` argument from `biome lsp-proxy` and `biome start` commands.

If you are using one of our official plugins for IDEs or editors, just update it to the latest version.

If you are a developer of a plugin, please update your plugin to use `workspace/configuration` response instead of
using `--config-path` argument. Biome's LSP will resolve a configuration in the workspace automatically, so it is
recommended to keep it empty unless you are using a custom configuration path.
