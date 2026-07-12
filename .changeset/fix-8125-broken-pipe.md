---
"@biomejs/biome": patch
---

Fixed [#8125](https://github.com/biomejs/biome/issues/8125): `EnvConsole` no longer panics with `failed to write markup to console` (or `failed to write to console`) when stdout/stderr is piped to a reader that closes early, e.g. `biome format ... | head`. Write errors with `io::ErrorKind::BrokenPipe` are now treated as a clean termination; other I/O errors still panic to surface unexpected conditions loudly.
