---
"@biomejs/biome": patch
---

Fixed [#13429](https://github.com/biomejs/biome/issues/13429): `biome_package::node_semver` now expands `^0.0.0`, `^0.0`, and `^0.0.0-<prerelease>` caret ranges with their proper lower and upper bounds (`>=0.0.0 <0.0.1-0`, `>=0.0.0 <0.1.0-0`, and `>=0.0.0-<prerelease> <0.0.1-0`) instead of `<1.0.0-0`.
