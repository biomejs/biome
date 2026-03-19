---
"@biomejs/biome": patch
---

Fixed whole-group lint enables so they no longer implicitly turn on domain-specific rules. Domain-tagged rules now require an explicit domain or rule selection.

For example, `linter.rules.correctness = "error"` no longer enables React- or Qwik-specific correctness rules unless `linter.domains.react`, `linter.domains.qwik`, or an explicit rule config also enables them.
