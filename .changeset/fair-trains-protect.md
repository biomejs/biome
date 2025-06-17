---
"@biomejs/biome": minor
---

Biome VCS integration now supports nested ignore files.

For `git`, if a `.gitignore` is found in a nested folder `root/packages/foo/`, and it contains the pattern `dist/`, only files and directories inside `root/packages/foo/dist` are matched.
