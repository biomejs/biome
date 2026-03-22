---
"@biomejs/biome": minor
---

Added support for detecting embedded GraphQL and CSS in untagged template literals via comment-based markers. Biome now recognizes `#graphql` and `#css` on the first line of a template literal, as well as `/* GraphQL */` and `/* CSS */` block comments before a template literal, as embedded language markers for formatting and linting. This is useful for codebases that cannot use tagged templates (e.g. `gql`) because they rely on `as const` for type inference with tools like graphql-codegen.

```js
const query = `#graphql
  query foo {
    bar
  }
` as const;

const query2 = /* GraphQL */`
  query foo {
    bar
  }
`;
```

Closes [#9511](https://github.com/biomejs/biome/discussions/9511).
