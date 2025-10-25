---
"@biomejs/biome": patch
---

Fixed [#7230](https://github.com/biomejs/biome/issues/7230): [`noUselessStringConcat`](https://biomejs.dev/linter/rules/no-useless-string-concat/) now correctly detects multiline strings with leading `+` operators (such as those from `operatorLinebreak="before"`).

#### Example
```ts
// This used to error depending on whether the `+` operators were at the start or end of lines.
// Now, it works in both cases.
const reallyLongString =
  "Lorem ipsum dolor sit amet consectetur adipiscing elit."
  + "Quisque faucibus ex sapien vitae pellentesque sem placerat."
  + "In id cursus mi pretium tellus duis convallis."
  + "Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla";
```
