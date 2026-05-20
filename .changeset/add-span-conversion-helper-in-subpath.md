---
"@biomejs/js-api": minor
---

Added `spanInBytesToSpanInCodeUnits` helper function in subpath exports of `@biomejs/js-api`.

```js
import { spanInBytesToSpanInCodeUnits } from "@biomejs/js-api/nodejs";
// Or:
// import { spanInBytesToSpanInCodeUnits } from "@biomejs/js-api/bundler";
// import { spanInBytesToSpanInCodeUnits } from "@biomejs/js-api/web";

const [start, end] = spanInBytesToSpanInCodeUnits(
    diagnostic.location.span,
    content
);
const text = content.slice(start, end); // Correctly extracts the text
```
