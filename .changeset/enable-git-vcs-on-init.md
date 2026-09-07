---
"@biomejs/biome": minor
---

Added automatic Git VCS integration when running `biome init` inside a Git repository, including from nested directories and Git worktrees. The generated configuration sets `vcs.enabled` and `vcs.useIgnoreFile` to `true`, with `vcs.clientKind` set to `"git"`.
