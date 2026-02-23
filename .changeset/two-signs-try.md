---
"@biomejs/biome": patch
---

Added `graphql` to valid embedded graphql template tags inside JavaScript files, when the feature experimentalEmbeddedSnippetsEnabled is enabled. This allows proper support for graphql tags used in RelayJS.
Following example will work:

```javascript
import { graphql } from "@biomejs/biome";

const query = graphql`
  query {
    user(id: 1) {
      id
      name
    }
  }
`;
```
