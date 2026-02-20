---
"@biomejs/biome": minor
---

Added support for applying GritQL plugin rewrites as code actions. GritQL plugins that use the rewrite operator (`=>`) now produce fixable diagnostics for JavaScript, CSS, and JSON files. All plugin rewrites are treated as unsafe fixes and require `--write --unsafe` to apply. Without `--unsafe`, the rewrite is shown as an "Unsafe fix" suggestion in the diagnostic output.

**Example plugin** (`useConsoleInfo.grit`):
```grit
language js

`console.log($msg)` as $call where {
    register_diagnostic(span = $call, message = "Use console.info instead of console.log.", severity = "warning"),
    $call => `console.info($msg)`
}
```

Running `biome check --write --unsafe` applies the rewrite, transforming `console.log("hello")` into `console.info("hello")`.
