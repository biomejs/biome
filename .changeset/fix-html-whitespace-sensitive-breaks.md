---
"@biomejs/biome": patch
---

Fixed several places where the HTML formatter added or dropped whitespace that a browser renders.

Text that touches a closing tag now takes the tag's `>` with it when it has to start a new line, instead of having a space put in front of it:

```diff
  <div>
-   before<meter value=".5"></meter>
-   after
+   before<meter value=".5"></meter
+   >after
  </div>
```

A closing tag that follows text is also weighed together with the last word, so the two move down a line together rather than running past the print width.

Blank lines between children survive a trailing space on the line above them, and blank lines between comments are no longer collapsed.

`<marquee>` is an inline-block element, and `<noscript>`, `<video>`, `<audio>` and `<object>` are plain inline elements, which is what the browser default stylesheet says.
