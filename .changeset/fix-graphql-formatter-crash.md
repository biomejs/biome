---
"@biomejs/biome": patch
---

Fixed [#9131](https://github.com/biomejs/biome/issues/9131): GraphQL formatter now properly tracks source positions when formatting block strings with multiple lines. 

Before this fix, the formatter would crash when processing block strings with empty or blank lines because position tracking was skipping these lines, causing incorrect source position calculations.

```graphql
query sellerMetadata {
  getSellerMetadata(
    description: """
    First line

    Third line
        Fourth line with indent
    """
  ) {
    marketplaceId
  }
}
```

The above code would previously crash. Now it formats correctly, with empty lines preserved via `empty_line()` and source positions accurately tracked for IDE integrations.
