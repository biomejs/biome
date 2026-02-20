---
"@biomejs/biome": minor
---

Added support for applying GritQL plugin rewrites as code actions. GritQL plugins that use the rewrite operator (`=>`) now produce fixable diagnostics for JavaScript, CSS, and JSON files. By default, plugin rewrites are treated as unsafe fixes and require `--write --unsafe` to apply. Plugin authors can pass `fix_kind = "safe"` to `register_diagnostic()` to mark a fix as safe, allowing it to be applied with just `--write`.

**Example plugin** (`useConsoleInfo.grit`):
```grit
language js

`console.log($msg)` as $call where {
    register_diagnostic(span = $call, message = "Use console.info instead of console.log.", severity = "warn", fix_kind = "safe"),
    $call => `console.info($msg)`
}
```

Running `biome check --write` applies safe rewrites. Unsafe rewrites (the default, or `fix_kind = "unsafe"`) still require `--write --unsafe`.
