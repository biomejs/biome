---
"@biomejs/biome": patch
---

Fixed [#7304](https://github.com/biomejs/biome/issues/7304): the HTML formatter now preserves authored segment breaks between CJK characters, and next to CJK punctuation, instead of replacing them with spaces.

```diff
 <div lang="zh-Hant-TW">
-  這個段落是那麼長， 在一行寫不行。
+  這個段落是那麼長，
+  在一行寫不行。
 </div>
```
