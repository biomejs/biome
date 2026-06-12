---
"@biomejs/biome": minor
---

Added the new assist action [`useSortedEnumMembers`](https://biomejs.dev/assist/actions/use-sorted-enum-members/), which sorts TypeScript & GraphQL enum members.

**Invalid**:

```graphql
enum Role {
  SUPER_ADMIN
  ADMIN
  USER
  GOD
}
```
