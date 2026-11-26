---
"@biomejs/biome": patch
---

Fixed [#13386](https://github.com/biomejs/biome/issues/13386): Biome no longer reports "Expected a list item" after the closing fence of a code block nested in a list item inside a block quote, when another list item follows. A bullet that starts the next line, such as a second `- > -` item, now stays in the outer list.
