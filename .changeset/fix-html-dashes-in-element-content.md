---
"@biomejs/biome": patch
---

Fixed [#9238](https://github.com/biomejs/biome/issues/9238): The HTML parser no longer incorrectly reports `---` inside element content (e.g. `<td>---</td>`) as an "Unexpected value or character" error.
