---
"@biomejs/biome": patch
---

Fixed [#12042](https://github.com/biomejs/biome/issues/12042): the HTML formatter no longer removes the space between text and an inline element on the next line when it joins the lines.

```diff
- <p>a <em>b</em> c<u>d</u></p>
+ <p>a <em>b</em> c <u>d</u></p>
```
