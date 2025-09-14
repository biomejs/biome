---
"@biomejs/biome": patch
---

Fixed [#7192](https://github.com/biomejs/biome/issues/7192):
`noUnusedPrivateClassMembers` now treats private members in compound assignments (+=, -=, ??=, etc.) as used,
while plain assignments (=) do not count as usage.

Example that is now correctly flagged:

```typescript
class App {
  #persistenceRequest: Promise<boolean> | undefined;
  saveData() {
    this.#persistenceRequest +=  2;
  }
}
```
