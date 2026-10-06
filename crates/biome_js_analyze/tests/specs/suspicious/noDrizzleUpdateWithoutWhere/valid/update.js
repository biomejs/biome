/* should not generate diagnostics */

// update with set and where
await db.update(users).set({ name: "John" }).where(eq(users.id, 1));

// update with where, no await
db.update(users).set({ name: "John" }).where(eq(users.id, 1));

// update with where stored in variable
const result = db.update(users).set({ active: false }).where(eq(users.id, id));

// not a drizzle object (not in options)
await database.update(users).set({ name: "John" });
await orm.update(users).set({ name: "John" });

// not an update call
await db.select().from(users);
await db.insert(users).values({ name: "John" });
