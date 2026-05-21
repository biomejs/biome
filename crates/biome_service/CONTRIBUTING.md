# Biome Service

The Biome Service is where we implement the [Workspace](src/workspace.rs).

The workspace is where Biome keeps all internal state of projects, such as open
documents, but also more advanced service data, such as the instances of our
[project layout](../biome_project_layout/) and our
[module graph](../biome_module_graph/).

Note that there are two implementations of the `Workspace` trait:

* [`WorkspaceServer`](src/workspace/server.rs) maintains the state itself, and
  is used both inside the daemon as well as in the CLI when it runs in
  daemonless mode.
* [`WorkspaceClient`](src/workspace/client.rs) is used for creating connections
  to the daemon and communicating with the `WorkspaceServer`.

## Watcher

The state inside our workspace is kept in sync with the filesystem using the
[`WorkspaceWatcher`](src/workspace_watcher.rs). The watcher is only active in
daemon mode and not used by the CLI.

### Debugging

Debugging the watcher can be tricky, because you need to run the daemon,
interact with the filesystem, and observe the daemon's state somehow.

Debugging is possible with the VS Code extension or Zed extension, but for an easier experience,
you can use these CLI commands:

1. Start the daemon using `cargo run --bin=biome -- start`. Note there won't be
   much output to inspect, but the daemon will write its logs in your
   [cache folder](../../crates/biome_fs/src/dir.rs).
2. Run commands against the daemon, such as
   `cargo run --bin=biome -- lint --use-server <path>`.

The rule `noImportCycles` is currently the best candidate to observe the state
in the project layout and the module graph.

### Tests

The watcher has tests related to its workspace methods in
[`watcher.tests.rs`](src/workspace/watcher.tests.rs), but there are also more
end-to-end tests inside the [LSP tests](../biome_lsp/src/server.tests.rs).

## Add a new language to the Workspace

New languages must be added via [Rust features](https://doc.rust-lang.org/cargo/reference/features.html). This is necessary
to keep the core lean and small. The `biome_service` is used in many areas of the toolchain (tests, codegen, benchmarks, etc.),
so we want to pull the features we need.

Features **must opt in**. When the support for a new language is ready, the feature must be added in the `biome_cli`, `biome_lsp` and `biome_wasm`.

The feature must have the prefix `lang_*` e.g. `lang_yaml`, and it usually involves multiple crates:
- `biome_configuration_macros`
  ```toml
  lang_yaml = ["dep:biome_yaml_syntax", "dep:biome_yaml_analyze"]
  ```
- `biome_configuration`
  ```toml
  lang_yaml = ["biome_configuration_macros/lang_yaml"]
  ```
- `biome_service`
  ```toml
  lang_yaml = [
    "dep:biome_yaml_syntax",
    "dep:biome_yaml_analyze",
    "dep:biome_yaml_parser",
    "dep:biome_yaml_formatter",
    "biome_configuration/lang_yaml"
  ]
  ```
- `biome_test_utils`
  ```toml
  lang_yaml = ["biome_service/lang_yaml"]
  ```
