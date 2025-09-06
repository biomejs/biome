---
"@biomejs/biome": patch
---

Fixed [#7323](https://github.com/biomejs/biome/issues/7323). [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) no longer reports as unused TypeScript `private` members if the rule encounters a computed access on `this`.

In the following example, `member` as previously reported as unused.
It is no longer reported.

```ts
class TsBioo {
  private member: number;

  set_with_name(name: string, value: number) {
    this[name] = value;
  }
}
```
