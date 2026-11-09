---
"@biomejs/biome": patch
---

Fixed [#12980](https://github.com/biomejs/biome/issues/12980): the CSS parser no longer panics on a `composes` declaration whose value contains a non-identifier token, such as `composes: 1px;`. It reports a diagnostic instead.
