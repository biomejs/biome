---
"@biomejs/biome": patch
---

Fixed [#12020](https://github.com/biomejs/biome/issues/12020): the HTML parser now reports an error for a mismatched closing tag like the `</span>` in `<p>two</span></p>`. Previously, it accepted `</span>` as the end of `<p>` because `span` contains the letter `p`, and the formatter deleted everything after it.

A closing tag with no opening tag at the top level of a file, like the second `</p>` in `<p>a</p></p><p>b</p>`, is now also reported as an error, instead of the formatter deleting it and everything after it.
