---
"@biomejs/biome": patch
---

Fixed the HTML formatter treating `<marquee>` as an inline element, which kept the whitespace around its content.

```diff
- <marquee behavior="alternate"> This text will bounce </marquee>
+ <marquee behavior="alternate">This text will bounce</marquee>
```
