---
"@biomejs/biome": patch
---

Fixed a bug where the HTML formatter collapsed the whitespace inside `<textarea>`, `<xmp>` and `<plaintext>`, changing what the page renders.

```diff
- <textarea>
-  line one
- line two </textarea>
+ <textarea>line one line two</textarea>
```

Biome now prints the content of these elements exactly as it appears in the source, matching the existing behavior for `<pre>`.
