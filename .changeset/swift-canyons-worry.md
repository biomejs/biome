---
"@biomejs/biome": patch
---

Fixed [#12317](https://github.com/biomejs/biome/issues/12317) and [#12984](https://github.com/biomejs/biome/issues/12984): VCS ignore matching now preserves logical paths for external and symlinked inputs, normalizes stdin paths, combines coexisting `.gitignore` and `.ignore` rules, respects nested ignore precedence and Git's repository exclude file, and reloads ignore state when ignore files change.
