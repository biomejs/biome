---
"@biomejs/biome": patch
---

Fixed [#8333](https://github.com/biomejs/biome/issues/8333): a crash and a wrong value when Biome's type inference unescaped a legacy octal escape in a string literal.

Legacy octal escapes are valid in sloppy-mode code, and any rule that reads a string literal's value reaches them. Biome used to panic on `"\01"`, and it read `"\1"` as `"1"` instead of the character with code point 1. Both now follow the Annex B semantics:

```js
"\01"; // no longer crashes; reads as "\u{01}"
"\1"; // now reads as "\u{01}" instead of "1"
```
