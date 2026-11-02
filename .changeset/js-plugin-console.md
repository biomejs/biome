---
"@biomejs/biome": patch
---

JavaScript plugins can now call `console.log()`, `console.info()`, `console.warn()`, `console.error()`, and `console.debug()` inside a rule's `run()` function. Like `log()` in GritQL plugins, each message is reported as a verbose diagnostic on the node passed to `run()`, so it is visible when Biome runs with `--verbose`. Nodes and tokens, including those inside arrays, are printed with their kind, range, and source text. Messages printed while the plugin module loads are discarded.
