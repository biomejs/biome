---
"@biomejs/biome": patch
---

Changed [`noUnusedPrivateClassMembers`](https://biomejs.dev/linter/rules/no-unused-private-class-members/) to align more fully with meaningful reads.

#### Invalid examples (previously valid)

```ts
class UsedMember {
  get #usedAccessor() {}
  set #usedAccessor(value) {}

  method() {
    // no return statement so no meaningful read
    this.#usedAccessor = 42;
  }
}

class UsedMember {
  #usedInInnerClass;

  method(a) {
    return class {
      // not really used, a is not reference to this scope
      foo = a.#usedInInnerClass;
    }
  }
}

class UsedMember {
  set #accessorUsedInMemberAccess(value) {} // <- unused

  method(a) {
    // there is no getter, so this is not a read at all
    [this.#accessorUsedInMemberAccess] = a;
  }
}

class UsedMember {
  #usedInInnerClass;

  method(a) {
    return class {
      foo = a.#usedInInnerClass;
    }
  }
}

class C {
  set #x(value) {
    doSomething(value);
  }

  foo() {
    // no return statement so not a meaningful read.
    this.#x = 1;
  }
}
```

#### Valid examples

```js
class Foo {
  #usedOnlyInWriteStatement = 5;
  method() {
    // this is both meaningful read and write.
    this.#usedOnlyInWriteStatement += 42;
  }
}
```
