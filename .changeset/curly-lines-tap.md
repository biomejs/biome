---
"@biomejs/biome": patch
---

Fixed [#11529](https://github.com/biomejs/biome/issues/11529), where [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) missed unhandled Promise chains when the imported function's module belonged to an import cycle. Cyclic modules now preserve types for exports that do not participate in recursive type dependencies.
