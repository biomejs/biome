/* should generate diagnostics */

import { db } from "./db";

// query on the database object inside the callback
await db.transaction(async (tx) => {
	await db.insert(users).values({ name: "John" });
});

// non-async callback
db.transaction(function () {
	return db.select().from(users);
});

// passed as an argument
await db.transaction(async (tx) => {
	await createUser(db, "John");
});

// passed as a shorthand property
await db.transaction(async (tx) => {
	await createUser({ db, name: "John" });
});

// inside a nested function
await db.transaction(async (tx) => {
	await Promise.all(ids.map((id) => db.delete(users).where(eq(users.id, id))));
});

// nested transaction started on the database object
await db.transaction(async (tx) => {
	await db.transaction(async (tx2) => {});
});

// inside a nested transaction started on the transaction object
await db.transaction(async (tx) => {
	await tx.transaction(async (tx2) => {
		await db.select().from(users);
	});
});

// any configured object name, declared outside the file
await database.transaction(async (tx) => {
	await database.select().from(users);
});
