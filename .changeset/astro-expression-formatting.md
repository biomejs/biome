---
"@biomejs/biome": patch
---

Astro `{...}` template and attribute expressions are now formatted instead of being left as written. Short expressions stay on one line, longer ones break and indent with the markup around them.

```diff
- <p>{ items . map( ( item )=>( <li class = 'item' >{item}</li> ) ) }</p>
+ <p>{items.map((item) => <li class="item">{item}</li>)}</p>
```

The children of `<script>`, `<style>` and `is:raw` elements inside an expression keep their text byte for byte.
