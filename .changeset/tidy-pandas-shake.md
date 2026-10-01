---
"@biomejs/biome": patch
---

Fixed [#12057](https://github.com/biomejs/biome/issues/12057). The [useSortedKeys](https://biomejs.dev/assist/actions/use-sorted-keys/javascript) assist no longer swallows a sorted property into a trailing `//` comment. When sorting would move a property onto the same line after such a comment, the safe fix now breaks the line after the comment so the following property starts on its own line.
