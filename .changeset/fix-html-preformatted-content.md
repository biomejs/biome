---
"@biomejs/biome": patch
---

Fixed a bug where the HTML formatter collapsed the whitespace inside `<textarea>`, `<xmp>` and `<plaintext>`, changing what the page renders. Because `<textarea>` is rendered with `white-space: pre-wrap`, its text is the value of the form field, so the newlines were being edited out of the field:

```diff
- <textarea>
-  line one
- line two </textarea>
+ <textarea>line one line two</textarea>
```

Biome now prints the content of these elements exactly as it appears in the source, matching the existing behavior for `<pre>`.
