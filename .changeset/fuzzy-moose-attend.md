---
"@biomejs/biome": patch
---

Added the nursery rule [`noDrizzleDbObjectInsideTransaction`](https://biomejs.dev/linter/rules/no-drizzle-db-object-inside-transaction/), which reports a Drizzle ORM instance used inside its own transaction callback instead of the transaction object.

```js
await db.transaction(async (tx) => {
  await db.insert(users).values({ name: "John" });
});
```
