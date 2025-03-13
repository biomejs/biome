---
"@biomejs/biome": major
---

Removed the `--config-path` argument from the `biome lsp-proxy` and `biome start` commands.

The option was overriding the configuration path for all workspaces opened in the Biome daemon, which led to a configuration mismatch problem when multiple projects are opened in some editors or IDEs.

If you are using one of our official plugins for IDEs or editors, it is recommended to update it to the latest version of the plugin, or you will get unexpected behavior.

If you are a developer of a plugin, please update your plugin to use the `workspace/configuration` response instead of using the `--config-path` argument. Biome's LSP will resolve a configuration in the workspace automatically, so it is recommended to keep it empty unless you are using a custom configuration path.
