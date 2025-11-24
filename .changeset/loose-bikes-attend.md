---
"@biomejs/biome": patch
---

Fixed bugs in the HTML parser so that it will flag invalid shorthand syntaxes instead of silently accepting them. For example, `<Foo : foo="5" />` is now invalid because there is a space after the `:`.
