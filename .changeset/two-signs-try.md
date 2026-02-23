---
"@biomejs/biome": patch
---

Added `graphql` to valid embedded graphql template tags inside JavaScript files, when the feature `javascript.experimentalEmbeddedSnippetsEnabled` is enabled. This allows proper support for graphql tags used in RelayJS.

Now, code snippets like the following are correctly formatted and limited:

```js
import { graphql } from "react-relay";

const query = graphql`
  query {
    user(id: 1) {
      id
      name
    }
  }
`;
```
