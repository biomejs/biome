---
"@biomejs/biome": minor
---

Added the new assist action [`useSortedEnumMembers`](https://biomejs.dev/assist/actions/use-sorted-enum-members/), which sorts TypeScript & GraphQL enums members.

**Invalid**:

```graphql
enum Role {
  SUPER_ADMIN
  ADMIN
  USER
  GOD
}
```
