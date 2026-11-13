---
"@biomejs/biome": patch
---

Fixed [#13107](https://github.com/biomejs/biome/issues/13107): [`noDistractingElements`](https://biomejs.dev/linter/rules/no-distracting-elements/) now removes the whole paired element with its unsafe fix. Previously, applying the fix to `<marquee>content</marquee>` removed only the opening tag, leaving a stray closing tag that broke the file's syntax.
