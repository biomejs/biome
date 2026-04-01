/* should generate diagnostics */

// simple delete without where
await db.delete(users);
await nested.db.delete(users);
await context.nested.db.delete(users);

// delete without where, no await
db.delete(users);
nested.db.delete(users);
context.nested.db.delete(users);

// delete assigned to variable without where
const result = db.delete(users);
const result2 = nested.db.delete(users);
const result3 = context.nested.db.delete(users);

// .where as property access (not called) should still trigger
db.delete(users).where;
nested.db.delete(users).where;
context.nested.db.delete(users).where;
