/* should generate diagnostics */

// simple delete without where
await db.delete(users);

// delete without where, no await
db.delete(users);

// delete assigned to variable without where
const result = db.delete(users);

// .where as property access (not called) should still trigger
db.delete(users).where;
