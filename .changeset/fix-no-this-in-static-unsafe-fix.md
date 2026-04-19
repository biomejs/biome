---
"@biomejs/biome": patch
---

Fixed [#10011](https://github.com/biomejs/biome/issues/10011): The code fix for [`noThisInStatic`](https://biomejs.dev/linter/rules/no-this-in-static/) is now marked as unsafe. Replacing `this` with the class name changes behavior when the static method is called on a subclass (e.g. `new this(...)` inside a factory method returns the subclass, while `new ClassName(...)` always returns the base class).
