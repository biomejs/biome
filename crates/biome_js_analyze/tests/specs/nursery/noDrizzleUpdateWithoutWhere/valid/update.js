/* should not generate diagnostics */

// update with set and where
await db.update(users).set({ name: "John" }).where(eq(users.id, 1));
await nested.db.update(users).set({ name: "John" }).where(eq(users.id, 1));
await context.nested.db.update(users).set({ name: "John" }).where(eq(users.id, 1));

// update with where, no await
db.update(users).set({ name: "John" }).where(eq(users.id, 1));
nested.db.update(users).set({ name: "John" }).where(eq(users.id, 1));
context.nested.db.update(users).set({ name: "John" }).where(eq(users.id, 1));

// update with where stored in variable
const result = db.update(users).set({ active: false }).where(eq(users.id, id));
const result2 = nested.db.update(users).set({ active: false }).where(eq(users.id, id));
const result3 = context.nested.db.update(users).set({ active: false }).where(eq(users.id, id));

// not a drizzle object (not in options)
await database.update(users).set({ name: "John" });
await orm.update(users).set({ name: "John" });
await context.nested.database.update(users).set({ name: "John" });
await context.nested.orm.update(users).set({ name: "John" });

// not an update call
await db.select().from(users);
await db.insert(users).values({ name: "John" });
await nested.db.select().from(users);
await nested.db.insert(users).values({ name: "John" });
await context.nested.db.select().from(users);
await context.nested.db.insert(users).values({ name: "John" });
