---
"@biomejs/biome": patch
---

Fixed [#7230](https://github.com/biomejs/biome/issues/7230): [`noUselessStringConcat`](https://biomejs.dev/linter/rules/no-useless-string-concat/) no longer emits false positives for multi-line strings with leading `+` operators.

Previously, the rule did not check for leading newlines on the `+` operator, emitting false positives if one occurred at the start of a line. \
Notably, formatting with `operatorLinebreak="before"` would move the `+` operators to the start of lines automatically, resulting in spurious errors whenever a multi-line string was used.

Now, the rule correctly detects and ignores multi-line concatenations with leading operators as well, working regardless of the setting of `operatorLinebreak`.

**Example**
```ts
// The following code used to error if the `+` operators were at the start of lines (as opposed to the end).
// Now, the rule correctly recognizes this as a stylistic concatenation and ignores it.
const reallyLongStringThatShouldNotError =
  "Lorem ipsum dolor sit amet consectetur adipiscing elit."
  + "Quisque faucibus ex sapien vitae pellentesque sem placerat."
  + "In id cursus mi pretium tellus duis convallis."
  + "Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla";
```
