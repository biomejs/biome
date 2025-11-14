---
"@biomejs/biome": patch
---

Changed [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) to align more fully with meaningful reads.

This rule now distinguishes more carefully between writes and reads of private class members.

- A *meaningful read* is any access that affects program behavior.
- For example, `this.#x += 1` both reads and writes `#x`, so it counts as usage.
- Pure writes without a read (e.g. `this.#x = 1` with no getter) are no longer treated as usage.

This change ensures that private members are only considered “used” when they are actually read in a way that influences execution.

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

