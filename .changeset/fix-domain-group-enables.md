---
"@biomejs/biome": patch
---

Fixed a bug where enabling the rules of a whole group, would enable rules that belonged to a domain under the same group.

For example, `linter.rules.correctness = "error"` no longer enables React- or Qwik-specific correctness rules unless `linter.domains.react`, `linter.domains.qwik`, or an explicit rule config also enables them, or their relative dependencies are installed.
