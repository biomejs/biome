---
"@biomejs/js-api": minor
---

Added a new `spanInBytesToSpanInCodeUnits` helper function to convert byte-based spans from Biome diagnostics to UTF-16 code unit spans.

Biome internally uses UTF-8 byte offsets for spans, but JavaScript strings use UTF-16 code units. This causes incorrect text extraction when using `string.slice()` with non-ASCII content. The new helper function correctly handles this conversion, including surrogate pairs and unpaired surrogates.

```js
import { spanInBytesToSpanInCodeUnits } from "@biomejs/js-api";

const [start, end] = spanInBytesToSpanInCodeUnits(
    diagnostic.location.span,
    content
);
const text = content.slice(start, end); // Correctly extracts the text
```
