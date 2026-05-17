---
"@biomejs/biome": patch
---

Fixed multiple issues in how `vcs.useIgnoreFile` evaluates `.gitignore` files:

- Files inside an ignored directory are now correctly ignored when their full path is checked directly. Previously, a `.gitignore` rule like `node_modules` only matched the directory itself, not files beneath it (an issue with absolute paths sent by the LSP and with symlinked dependency trees).
- Nested `.gitignore` files now correctly override their parents. A negation pattern (`!path`) in a deeper `.gitignore` is no longer overridden by an exclude pattern in the project root `.gitignore`.
