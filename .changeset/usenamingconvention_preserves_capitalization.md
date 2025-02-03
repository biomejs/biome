---
"@biomejs/biome": patch
---

The `useNamingConvention` rule now suggests a rename that preserves uppercase if possible.

For instance, Biome suggested renaming `HTMLWrapper` as `htmlWrapper`:

```diff
- import HTMLWrapper from "HTMLWrapper.tsx";
+ import htmlWrapper from "HTMLWrapper.tsx";

  function component() {
-   return <HTMLWrapper> </HTMLWrapper>;
+   return <htmlWrapper> </HTMLWrapper>;
  }
```

Since both `PascalCase` and `CamelCase` are accepted, Biome now suggests renaming `HTMLWrapper` as `HtmlWrapper`:

```diff
- import HTMLWrapper from "HTMLWrapper.tsx";
+ import HtmlWrapper from "HTMLWrapper.tsx";

  function component() {
-   return <HTMLWrapper> </HTMLWrapper>;
+   return <HtmlWrapper> </HTMLWrapper>;
  }
```
