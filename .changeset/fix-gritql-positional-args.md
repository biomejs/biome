---
"@biomejs/biome": patch
---

Fixed a bug where GritQL patterns rejected positional (unkeyed) arguments.

The GritQL spec explicitly allows omitting parameter names when arguments are passed in order, but Biome was rejecting anything other than named arguments (e.g., `console_method_to_info(`log`)` would fail while `console_method_to_info(method = `log`)` worked).

The fix operates at two layers: the parser now correctly distinguishes a positional pattern from a named argument using lookahead, and the pattern compiler now resolves non-variable positional arguments by position instead of rejecting them outright. This applies to user-defined patterns, functions, predicates, and AST node patterns.
