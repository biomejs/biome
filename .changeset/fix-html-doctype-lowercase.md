---
"@biomejs/biome": patch
---

The HTML formatter now writes the HTML5 doctype in lowercase, matching Prettier:

```diff
- <!DOCTYPE html>
+ <!doctype html>
```

This only applies to a plain `.html` file whose doctype stands alone. A doctype that names a DTD keeps the case it was written with, since the rest of the declaration is not lowercased either:

```html
<!DOCTYPE html PUBLIC "-//W3C//DTD HTML 4.01//EN" "http://www.w3.org/TR/html4/strict.dtd">
```

A `.vue`, `.svelte`, or `.astro` file keeps whatever the author wrote.
