---
"@biomejs/biome": patch
---

Fixed [#6287](https://github.com/biomejs/biome/issues/6287) where the Biome LSP didn't adhere to the `settings.requireConfiguration` option when pulling diagnostics and code actions.
Note that for this configuration be correctly applied, your editor must support dynamic registration capabilities.
