---
"@biomejs/biome": patch
---

The GraphQL formatter no longer moves a comment written between a type system definition's name and its `{` onto the name's line, where reformatting the result would move it again and reorder it against its neighbours.

```graphql
type Query
# the first field
# and a second comment
{ a: a }
```

Before, the first run produced `type Query # the first field` and the second run `type Query # and a second comment # the first field`. Now the comments stay inside the braces, in the order they were written, and the output does not change on a second run:

```graphql
type Query {
	# the first field
	# and a second comment
	a: a
}
```

This covers `type`, `interface`, `input`, `enum` and `schema`.
