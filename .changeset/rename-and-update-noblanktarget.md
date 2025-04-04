---
"@biomejs/biome": major
---

Fixed [#5495](https://github.com/biomejs/biome/issues/5495): The rule
[`noBlankTarget`](https://biomejs.dev/linter/rules/no-blank-target/) has been
updated to accept the `rel="noopener"` in addition to `rel="noreferrer"`.
In addition, an option has been added that allows `rel="noreferrer"` to be
disabled.

The rule has been moved from the `a11y` group to the `security` group.
