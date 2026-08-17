---
"@biomejs/biome": patch
---

Fixed parent tag wrapping when an HTML element starts or ends with a block-like or hidden child such as `source`, `track`, or `param`.

```diff
- <video src="brave.webm"><track kind="subtitles" src="brave.en.vtt"></video>
+ <video src="brave.webm">
+   <track kind="subtitles" src="brave.en.vtt">
+ </video>
```
