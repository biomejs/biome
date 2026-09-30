---
"@biomejs/biome": patch
---

The HTML formatter no longer moves the text after an inline element onto its own line when a second run reformats a file it has already formatted. Formatting

```html
<div>
aaa bbb ccc ddd eee fff ggg hhh iii jjj kkk lll mmm nnn ooo <strong>x</strong> yyy
</div>
```

wraps the text and leaves `<strong>x</strong> yyy` together, but running the formatter again used to break the line after `</strong>` as well, so the output was never stable.
