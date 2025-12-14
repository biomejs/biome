---
"@biomejs/biome": patch
---

Fix `--stdin-file-path` being blocked by `includes` configuration. Stdin input now bypasses all path-based ignore checks (`files.includes`, `formatter.includes`, VCS ignore). The path is only used for language detection, since stdin content is explicitly provided by the user.
