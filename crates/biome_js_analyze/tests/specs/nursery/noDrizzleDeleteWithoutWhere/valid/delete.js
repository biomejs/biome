/* should not generate diagnostics */

// delete with where
await db.delete(users).where(eq(users.id, 1));
await nested.db.delete(users).where(eq(users.id, 1));
await context.nested.db.delete(users).where(eq(users.id, 1));

// delete with where, no await
db.delete(users).where(eq(users.id, 1));
nested.db.delete(users).where(eq(users.id, 1));
context.nested.db.delete(users).where(eq(users.id, 1));


// delete with where stored in variable
const result = db.delete(users).where(eq(users.id, id));
const result2 = nested.db.delete(users).where(eq(users.id, id));
const result3 = context.nested.db.delete(users).where(eq(users.id, id));

// not a drizzle object (not in options)
await database.delete(users);
await orm.delete(users);
await context.nested.database.delete(users);
await context.nested.orm.delete(users);

// not a delete call
await db.select().from(users);
await db.insert(users).values({ name: "John" });
await nested.db.select().from(users);
await nested.db.insert(users).values({ name: "John" });
await context.nested.db.select().from(users);
await context.nested.db.insert(users).values({ name: "John" });
