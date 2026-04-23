---
"@biomejs/biome": patch
---

Fixed [#10034](https://github.com/biomejs/biome/issues/10034): [`noUselessEscapeInRegex`](https://biomejs.dev/linter/rules/no-useless-escape-in-regex/) no longer flags escapes of `ClassSetReservedPunctuator` characters (`&`, `!`, `#`, `%`, `,`, `:`, `;`, `<`, `=`, `>`, `@`, `` ` ``, `~`) inside `v`-flag character classes as useless. These characters are reserved as individual code points in `v`-mode, so the escape is required.

The following pattern is now considered valid:

```js
/[a-z\&]/v;
```
