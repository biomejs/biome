---
"@biomejs/biome": patch
---

Fixed [#12724](https://github.com/biomejs/biome/issues/12724): when `proseWrap` is `"always"` or `"never"`, the Markdown formatter now wraps the text inside emphasis, strong emphasis, and strikethrough like the surrounding prose, instead of keeping their source line breaks and forcing line breaks around them. Text inside block quotes keeps its current formatting. With a `lineWidth` of 50:

```diff
- Some words before the strong text,
- **stop and tell
- them**
- after the strong text.
+ Some words before the strong text, **stop and tell
+ them** after the strong text.
```
