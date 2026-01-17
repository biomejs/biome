---
"@biomejs/biome": patch
---

Added the nursery rule [`useInlineScriptId`](https://biomejs.dev/linter/rules/use-inline-script-id/) to the Next.js domain.
This rule enforces `id` attribute on `next/script` components with inline content or `dangerouslySetInnerHTML`.

The following code is invalid:

```jsx
import Script from 'next/script';

export default function Page() {
  return (
    <Script>{`console.log('Hello');`}</Script> // must have `id` attribute
  );
}
```
