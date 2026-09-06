---
"@biomejs/biome": minor
---

Added a `--git` flag to `biome init`. The flag enables Git VCS integration and sets `vcs.useIgnoreFile` to `true` in the generated configuration, even when the command runs outside a Git repository.
