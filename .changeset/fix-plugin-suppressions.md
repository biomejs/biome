---
"@biomejs/biome": patch
---

Fixed [#12600](https://github.com/biomejs/biome/issues/12600): `biome lint --suppress` now adds suppression comments for diagnostics emitted by plugins, such as ``// biome-ignore lint/plugin/noFoo: ignored using `--suppress` ``. Biome now reports an error when a plugin file name contains whitespace, `:`, or `(`, because the file name is the plugin name in suppression comments and the comment parser can't read those characters.
