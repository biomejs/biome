---
"@biomejs/biome": patch
---

Fixed `biome format --stdin-file-path` corrupting single-codepoint non-ASCII characters when stdout is redirected.
