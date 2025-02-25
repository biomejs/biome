---
"@biomejs/biome": major
---

The Biome daemon now reuses its workspace across connections. This allows multiple clients to
reuse the same documents and other cached data that we extract from them.

This primarily affects our IDE extensions: If you open multiple IDEs/windows for the same project,
they'll connect to the same daemon and reuse each other's workspace.

The Biome CLI is unaffected unless you opt in with the `--use-server` argument.
