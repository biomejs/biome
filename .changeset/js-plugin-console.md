---
"@biomejs/biome": patch
---

JavaScript plugins can now use `console` inside a rule's `run()` function. Like `log()` in GritQL plugins, each message is reported as a verbose diagnostic on the node passed to `run()`, so it is visible when Biome runs with `--verbose`. Nodes and tokens, including those inside arrays, are printed with their kind, range, and source text, and the same description is returned by their new `toString()` method. Messages printed while the plugin module loads are discarded.
