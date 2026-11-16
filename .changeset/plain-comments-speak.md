---
"@biomejs/biome": patch
---

Added trivia APIs to JavaScript plugins. `leadingTrivia()` and `trailingTrivia()` on nodes and tokens return the whitespace, line breaks, and comments around them, and `leadingComments()` and `trailingComments()` on tokens return only the comments. Each piece has an `isComment()` method. `registerDiagnostic()` now also accepts tokens and trivia pieces, and highlights only that element.

```js
const keyword = statement.declaration.token("kindToken");
for (const comment of keyword.leadingComments()) {
  if (comment.text.includes("TODO")) {
    registerDiagnostic(comment, "warning", "Resolve this TODO.");
  }
}
```
