---
"@biomejs/biome": patch
---

Fixed [#11817](https://github.com/biomejs/biome/issues/11817): Biome no longer aborts with `SIGABRT` and a core dump when the pipe it writes diagnostics to is closed early, for example `biome check | head` or quitting a pager before the output ends.

A closed pipe is now treated as end of output, and the panic handler no longer writes to a stream that has already failed, so a single `EPIPE` can no longer escalate into a double panic.
