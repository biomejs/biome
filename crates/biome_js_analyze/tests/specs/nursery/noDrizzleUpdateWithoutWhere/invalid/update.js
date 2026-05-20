/* should generate diagnostics */

// update with set but no where
await db.update(users).set({ name: "John" });
await nested.db.update(users).set({ name: "John" });
await context.nested.db.update(users).set({ name: "John" });

// update without where, no await
db.update(users).set({ name: "John" });
nested.db.update(users).set({ name: "John" });
context.nested.db.update(users).set({ name: "John" });

// update assigned to variable without where
const result = db.update(users).set({ active: false });
const result2 = nested.db.update(users).set({ active: false });
const result3 = context.nested.db.update(users).set({ active: false });

// .where as property access (not called) should still trigger
db.update(users).set({ name: "John" }).where;
nested.db.update(users).set({ name: "John" }).where;
context.nested.db.update(users).set({ name: "John" }).where;
