---
"@biomejs/biome": minor
---

It's now possible to provide the stacktrace for a fatal error. The stacktrace is only available when the environment variable `RUST_BACKTRACE=1` is set, either via the CLI or exported `$PATH`. This is useful when providing detailed information for debugging purposes:

```shell
RUST_BACKTRACE=1 biome lint
```
