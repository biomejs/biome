---
"@biomejs/biome": patch
---

Fixed [#12334](https://github.com/biomejs/biome/issues/12334): Grit plugins can now name CSS/SCSS nodes that contain `List` in their name, such as `CssPseudoClassFunctionSelectorList` (the selector list inside `:is()`, `:where()`, `:not()` and `:matches()`).

The Grit mapping codegen filtered out every AST node whose name contained `"List"`, intending to skip list nodes. But `AstSrc` already keeps lists and unions in dedicated fields, so the node list only ever holds regular nodes — the filter therefore only dropped regular nodes that happened to have `List` in their name, making them unusable in Grit patterns. Such a plugin previously failed to compile with `UnknownFunctionOrPattern`.
