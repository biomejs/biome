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
