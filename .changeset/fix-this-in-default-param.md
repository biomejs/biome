---
"@biomejs/biome": patch
---

Fixed [#13084](https://github.com/biomejs/biome/issues/13084): The [`useThisInClassMethods`](https://biomejs.dev/linter/rules/use-this-in-class-methods/) rule now correctly recognizes `this` when used inside parameter default values for methods, setters, and class fields.
