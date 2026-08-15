---
"@biomejs/biome": patch
---

Fixed the HTML formatter's whitespace handling for `marquee`, `noscript`, `video`, `audio`, and `object` elements.

```diff
- <marquee behavior="alternate"> This text will bounce </marquee>
+ <marquee behavior="alternate">This text will bounce</marquee>
```
