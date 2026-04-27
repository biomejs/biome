---
"@biomejs/biome": minor
---

Biome now applies Git's local exclude file when VCS ignore files are enabled. Files listed in `.git/info/exclude` are skipped the same way as files listed in `.gitignore`, including in linked worktrees.
