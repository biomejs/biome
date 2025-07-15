---
"@biomejs/biome": patch
---

Fixed a bug where the root ignore file wasn't correctly loaded during the scanning phase, causing false positives and incorrect expectations among users.

Now, when using `vcs.useIgnoreFile`, the **the globs specified in the ignore file from the project root** will have the same semantics as the `files.includes` setting of the root configuration.

Refer to the [relative web page](https://biomejs.dev/internals/architecture/#configuring-the-scanner) to understand how they work.
