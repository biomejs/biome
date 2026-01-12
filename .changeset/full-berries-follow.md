---
"@biomejs/biome": minor
---

Added support for formatting and linting embedded GraphQL snippets in JavaScript.

For example, the following snippets are now formatted:

```js
import gql from "graphql-tag";

const PeopleCountQuery = gql`
  query PeopleCount {
    allPeople {
      totalCount
    }
  }
`;
```

```js
import { graphql } from "./graphql";

const PeopleCountQuery = graphql(`
  query PeopleCount {
    allPeople {
      totalCount
    }
  }
`);
```

This feature is experimental and must be enabled explicitly in the configuration:

```json
{
  "javascript": {
    "experimentalEmbeddedSnippetsEnabled": true
  }
}
```
