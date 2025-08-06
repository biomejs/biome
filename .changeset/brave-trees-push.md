---
"@biomejs/biome": patch
---

Added a new **experimental option** that allows parsing of `.html` files that contain interpolation syntax.

```json5
// biome.json
{
  "html": {
    // This is the new, experimental option.
    "parser": {
      "interpolation": true
    }
  }
}
```

```html
<h1>{{ $title }}</h1>
```
