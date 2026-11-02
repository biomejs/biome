---
"@biomejs/runtime": patch
---

`@biomejs/runtime` now ships only ES modules. CommonJS consumers must load it with `import()`, or use a Node.js version that supports `require()` of ES modules.
