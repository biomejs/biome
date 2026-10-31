---
"@biomejs/biome": patch
---

Fixed [#12332](https://github.com/biomejs/biome/issues/12332): [`noUnknownUnit`](https://biomejs.dev/linter/rules/no-unknown-unit/) no longer reports hex colors that start with a digit in SCSS custom properties or in unquoted legacy IE `filter` values. The rule now accepts `--accent: #4566a8;` instead of reporting the unknown unit `a8`.
