---
"@biomejs/biome": patch
---

Fixed [#12057](https://github.com/biomejs/biome/issues/12057). The [useSortedKeys](https://biomejs.dev/assist/actions/use-sorted-keys/javascript) assist no longer suggests a fix when sorting would move a property onto the same line after a `//` comment, which previously swallowed the property into the comment and silently changed runtime behavior.
