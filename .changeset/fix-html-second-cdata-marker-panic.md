---
"@biomejs/biome": patch
---

Fixed [#13258](https://github.com/biomejs/biome/issues/13258): the HTML parser no longer panics when a CDATA section starts with a second `<![CDATA[` marker. The marker is kept as literal text.
