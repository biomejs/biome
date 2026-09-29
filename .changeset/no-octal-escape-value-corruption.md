---
"@biomejs/biome": patch
---

Fixed a bug in [`noOctalEscape`](https://biomejs.dev/linter/rules/no-octal-escape/) where the safe fix could silently change a string's value. A legacy octal escape is at most two digits when the leading digit is `4`-`7`, so `"\751"` is `"\75"` + `"1"` (i.e. `"=1"`) but was rewritten to the single character `ǩ`; it is now rewritten to `"\x3d1"`.
