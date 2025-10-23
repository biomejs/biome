---
"@biomejs/biome": minor
---

Added a new option called `html.interpolation`. This option enables the parsing of text expressions (or interpolation) in HTML files.

The following `file.html` will be correctly formatted:

```html
<!-- file.html -->
<div>
  Hello {{ name }}!
  <p>Your balance is: {{ account.balance }}</p>
  <button>{{ isLoading ? "Loading..." : "Submit" }}</button>
</div>
```

To note that `html.interpolation` only parses text expressions that are delimited by double curly braces (`{{ }}`). The content of expressions is parsed as normal text.
