---
"@biomejs/biome": patch
---

Fixed [#12176](https://github.com/biomejs/biome/issues/12176): Type inference now resolves overloaded method signatures declared in interfaces and object types, the same way it already did for classes. [`useAwaitThenable`](https://biomejs.dev/linter/rules/use-await-thenable/) no longer reports awaiting a method whose matching overload returns a Promise, such as Fastify's `await app.listen({ port: 0 })`.
