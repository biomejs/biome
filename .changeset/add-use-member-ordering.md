---
"@biomejs/biome": patch
---

Added the nursery rule [`useMemberOrdering`](https://biomejs.dev/linter/rules/use-member-ordering/). The rule enforces a consistent ordering of class members, corresponding to ESLint TypeScript's `@typescript-eslint/member-ordering`.

```js
// Invalid: method before property
class Foo {
    method() {}
    name;
}
```

The default order is: index signatures, properties, accessors, constructors, methods, static blocks.

The order is fully configurable via a `groups` option that accepts an array of member group names in kebab-case (e.g., `index-signature`, `static-property`, `#private-method`, `get-accessor`, `static-block`). Members not listed in the configured order are allowed in any position. For example:

```json
{
  "linter": {
    "rules": {
      "nursery": {
        "useMemberOrdering": {
          "options": {
            "groups": ["constructor", "method", "property"]
          }
        }
      }
    }
  }
}
```
