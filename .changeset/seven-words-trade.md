---
"@biomejs/biome": patch
---

Added support for parsing and formatting the [CSS if function](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/if).

***Example***

``` css
.basic-style {
  color: if(style(--scheme: dark): #eeeeee; else: #000000;);
}
```
