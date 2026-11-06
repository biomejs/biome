---
"@biomejs/biome": patch
---

Fixed [#12980](https://github.com/biomejs/biome/issues/12980): the CSS parser no longer panics when a `composes` value contains a token that isn't a class name, such as `composes: 1px;` or `composes: a "s";`. It now reports a parse diagnostic instead.
