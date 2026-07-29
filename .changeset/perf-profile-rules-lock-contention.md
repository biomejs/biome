---
"@biomejs/biome": patch
---

Fixed a lock-contention bug in `--profile-rules`: every rule invocation contended on a single shared lock, which slowed down profiled runs under multi-threading and could make the reported per-rule timings misleading (occasional large `max` values unrelated to that rule's actual cost). Each thread now accumulates independently and results are merged when the profile is printed.
