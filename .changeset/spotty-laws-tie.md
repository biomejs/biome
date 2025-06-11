---
"@biomejs/biome": patch
---

Fixed an issue where the lexer didn't report errors for unterminated regex or string literals, such as the following cases:

```js
"string
'str
/\\217483
```