---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUselessParameterPropertyAssignment`](https://biomejs.dev/linter/rules/no-useless-parameter-property-assignment/), which reports constructor assignments such as `this.name = name` when `name` is a parameter property. TypeScript already sets the property to the parameter's value, so the assignment does nothing.

```ts
class User {
  constructor(public name: string) {
    this.name = name;
  }
}
```
