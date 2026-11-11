---
"@biomejs/biome": patch
---

Improved [`noUselessStringConcat`](https://biomejs.dev/linter/rules/no-useless-string-concat/). The rule now reports literals joined after another value, such as `foo + "a" + "b"`, and fixes template literals with placeholders, so `` `Hello, ${name}` + "!" `` becomes `` `Hello, ${name}!` ``.

The rule no longer reports strings that would join into text that looks like a placeholder, such as `"$" + "{name}"`, or into a directive such as `"use strict"`. It also no longer reports `obj.foo * "a" + "b"`, which its fix used to change to `obj.foo * "ab"`.

The fix no longer changes the value of the string in two cases. It used to change `1 + 2 + "b" + "c"` to `"12bc"` instead of `1 + 2 + "bc"`. It used to change `"\1" + "2"` to `"\12"`, which is a different escape sequence.

The fix no longer joins literals that are on different lines when the concatenation ends with two literals on the same line. It also no longer puts a line break from a template literal into a string literal, which made the code invalid.
