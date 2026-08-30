---
"@biomejs/biome": patch
---

Fixed [#7984](https://github.com/biomejs/biome/issues/7984): the [`useExplicitLengthCheck`](https://biomejs.dev/linter/rules/use-explicit-length-check/) unsafe fix no longer discards the line break, indentation, and comments that precede the expression it rewrites. Previously the rewritten expression was pulled onto the previous line, which commented it out when that line ended with a `//` comment.

```js
if (
  !showThinking && // while it's thinking there is hope to get suggestions
  !comments?.length
) {}
```
