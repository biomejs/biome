---
"@biomejs/biome": patch
---

Fixed [#12600](https://github.com/biomejs/biome/issues/12600): `biome lint --suppress` now adds suppression comments for diagnostics emitted by plugins, such as ``// biome-ignore lint/plugin/noFoo: ignored using `--suppress` ``.
