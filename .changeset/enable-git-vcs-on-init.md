---
"@biomejs/biome": minor
---

Added automatic Git VCS integration when running `biome init` inside a Git repository. The generated configuration enables `vcs.useIgnoreFile`, including when invoked from a nested directory or Git worktree.
