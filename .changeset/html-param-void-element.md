---
"@biomejs/biome": patch
---

Fixed `<param>` not being recognized as an HTML void element, so it is no longer formatted with a closing slash.

```diff
- <param name="movie" value="movie.mp4" />
+ <param name="movie" value="movie.mp4">
```
