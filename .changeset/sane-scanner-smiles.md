---
"@biomejs/biome": minor
---

Fixed [#6965](https://github.com/biomejs/biome/issues/6965): Implemented smarter scanner for project rules.

Previously, if project rules were enabled, Biome's scanner would scan all dependencies regardless of whether they were used by/reachable from source files or not. While this worked for a first version, it was far from optimal.

The new scanner first scans everything listed under the `files.includes` setting, and then descends into the dependencies that were discovered there, including transitive dependencies. This has two main advantages:
* Dependencies that are not reachable from your source files don't get indexed.
* Dependencies that have multiple type definitions, such as those with separate definitions for CommonJS and ESM imports, only have the relevant definitions indexed.

The change in the scanner also has a more nuanced impact: Previously, if you used `files.includes` to ignore a file in an included folder, the scanner would still index this file. Now the file is fully ignored, _unless you import it_.

As a user you should notice better scanner performance (if you have project rules enabled), and hopefully you need to worry less about configuring [`files.experimentalScannerIgnores`](https://biomejs.dev/reference/configuration/#filesexperimentalscannerignores). Eventually our goal is still to deprecate that setting, so if you're using it today, we encourage you to see which ignores are still necessary there, and whether you can achieve the same effect by ignoring paths using `files.includes` instead.

None of these changes affect the scanner if no project rules are enabled.
