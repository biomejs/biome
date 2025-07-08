---
"@biomejs/biome": minor
---

We have implemented a more targeted version of the scanner, which ensures that if you provide file paths to handle on the CLI, the scanner will exclude directories that are not relevant to those paths.

Note that for many commands, such as `biome check` and `biome format`, the file paths to handle are implicitly set to the current working directory if you do not provide any path explicitly. The targeted scanner also works with such implicit paths, which means that if you run Biome from a subfolder, other folders that are part of the project are automatically exempted.

Use cases where you invoke Biome from the root of the project without providing a path, as well as those where project rules are enabled, are not expected to see performance benefits from this.

Implemented [#6234](https://github.com/biomejs/biome/issues/6234), and fixed [#6483](https://github.com/biomejs/biome/issues/6483) and [#6563](https://github.com/biomejs/biome/issues/6563).
