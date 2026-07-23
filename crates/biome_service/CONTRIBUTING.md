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

New languages must be added via [Rust features](https://doc.rust-lang.org/cargo/reference/features.html). This keeps the
core lean because `biome_service` is used by the CLI, LSP, WASM bindings, tests, codegen, benchmarks, and helper crates.

Language features **must opt in**. A language feature must use the `lang_*` prefix, for example `lang_yaml`.
When the language is ready to ship, enable that feature from the entry points that should expose it, such as `biome_cli`,
`biome_lsp`, and `biome_wasm`.

`biome_languages` owns file source detection and the `DocumentFileSource` variants. Add the language feature there first:

```toml
lang_yaml = []
```

Then gate the source module, re-export, enum variant, conversion, and path/extension/language-id detection in
`biome_languages/src/lib.rs` behind `#[cfg(feature = "lang_yaml")]`.

`biome_service` must have a feature with the same name. That feature must enable the corresponding `biome_languages`
feature and every optional crate the service handler needs:

```toml
lang_yaml = [
  "biome_configuration/lang_yaml",
  "biome_languages/lang_yaml",
  "dep:biome_yaml_formatter",
  "dep:biome_yaml_parser",
  "dep:biome_yaml_syntax",
]
```

The service feature must also gate the language handler fields, initialization, modules, imports, and
`DocumentFileSource` match arms in `file_handlers/mod.rs`.

Keep `biome_languages/lang_*` and `biome_service/lang_*` aligned. Cargo features are scoped per package, so enabling
`biome_languages/lang_yaml` does not enable `biome_service/lang_yaml`. If any normal dependency enables
`biome_languages/lang_yaml` while `biome_service/lang_yaml` is off, `DocumentFileSource::Yaml(_)` exists but the service
handler match arm is compiled out, causing non-exhaustive matches.

Do not enable `biome_languages/lang_*` from parser, formatter, syntax, or analyzer crates unless their normal library code
really needs `DocumentFileSource` or file source detection. If only tests need those APIs, put `biome_languages` in
`[dev-dependencies]` with the language feature.

If schema generation exposes a `DocumentFileSource` variant through `biome_languages/schema`, make sure
`biome_service/schema` also enables the matching `biome_service/lang_*` feature. Otherwise `just gen-schema` can compile
`biome_languages` with the enum variant while compiling `biome_service` without the handler.

Other crates may need forwarding features, depending on the language:

```toml
# biome_configuration_macros, when the language has analyzer/configuration macro support
lang_yaml = ["dep:biome_yaml_analyze", "dep:biome_yaml_syntax"]

# biome_configuration
lang_yaml = ["biome_configuration_macros/lang_yaml"]

# biome_test_utils or other helper crates
lang_yaml = ["biome_service/lang_yaml"]
```
