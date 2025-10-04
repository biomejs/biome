---
"@biomejs/biome": patch
---

Changed [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) to align more fully with meaningful reads.

What’s a “meaningful read”?

A meaningful read happens when a private class member is actually read in a way that affects program behavior, not just written to.
For example, this.#x += 1 both reads and writes #x, so it counts as a meaningful read.
Assigning to a setter without reading it (e.g., this.#x = 1 with no getter) is not a meaningful read.

***Invalid examples (previously valid)***

```ts
class UsedMember {
  set #x(value) {
    doSomething(value);
  }

  foo() {
    // This assignment does not actually read #x, because there is no getter.
    // Previously, this was considered a usage, but now it’s correctly flagged.
    this.#x = 1;
  }
}
```

***Valid example (Previously invalid)***

```js
class Foo {
  #usedOnlyInWriteStatement = 5;

  method() {
    // This counts as a meaningful read because we both read and write the value.
    this.#usedOnlyInWriteStatement += 42;
  }
}
```

***Summary***
•	Only accesses that read a value are considered meaningful for the purpose of this rule.
•	Simple assignments to a setter without a corresponding getter no longer count as usage.
•	Operations like +=, method calls returning a value, or reading the property for computation are considered meaningful reads.
