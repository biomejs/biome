---
"@biomejs/biome": patch
---

Fixed [#12020](https://github.com/biomejs/biome/issues/12020): the HTML parser now reports an error for a mismatched closing tag like the `</span>` in `<p>two</span></p>`. Previously, it accepted `</span>` as the end of `<p>` because `span` contains the letter `p`, and the formatter deleted everything after it.
