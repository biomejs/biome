---
"@biomejs/biome": patch
---

Fixed [#12176](https://github.com/biomejs/biome/issues/12176): type-aware rules now select the matching overload of a method declared several times in an interface or object type. For example, [`useAwaitThenable`](https://biomejs.dev/linter/rules/use-await-thenable/) no longer reports `await app.listen({ port: 0 })` on a Fastify server, and still reports `await app.listen({ port: 0 }, callback)`.
