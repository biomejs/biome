/* should generate diagnostics */

// simple delete without where
await db.delete(users);

// delete without where, no await
db.delete(users);

// delete assigned to variable without where
const result = db.delete(users);

// different drizzle object name should NOT trigger (not in options)
await database.delete(users);
