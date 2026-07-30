---
"@biomejs/biome": patch
---

Fixed a stack overflow in type inference when a project depends on a package
whose declaration files form a large dependency cycle, such as an SDK whose
modules all reference each other through barrel files. Inferring one module of
such a cycle recursed into every other member on the Rust stack and crashed
workspace worker threads with `fatal runtime error: stack overflow`.

Nested whole-module inference is now bounded. Dependencies beyond the depth
limit are treated as unavailable, so affected types resolve to `Unknown`
instead of crashing, mirroring how imports blocked by cycle recovery behave.
