---
"@biomejs/biome": patch
---

Fixed [#10395](https://github.com/biomejs/biome/issues/10395): `biome format --stdin-file-path` no longer corrupts single-codepoint non-ASCII characters in string literals when stdout is not a TTY (for example when piping the output back into a file).

Previously, formatted stdin output was routed through the same console pipeline used for human-facing diagnostics, which substitutes certain Unicode characters with ASCII look-alikes for readability in plain-text terminals. This substitution was being incorrectly applied to the actual formatted source code as well, silently rewriting characters like `✔` to `√`.
