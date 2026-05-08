---
"@biomejs/biome": patch
---

Added the new nursery rule [`useThisInClassMethods`](https://biomejs.dev/linter/rules/use-this-in-class-methods/), based on ESLint's `class-methods-use-this`.

The rule now reports instance methods, getters, setters, and function-valued instance fields that do not use `this`, and `biome migrate eslint` preserves the supported `ignoreMethods`, `ignoreOverrideMethods`, and `ignoreClassesWithImplements` options.

**Invalid**:
```js
class Foo {
  bar() {
    // does not use `this`, invalid
    console.log("Hello Biome");
  }
}
```
