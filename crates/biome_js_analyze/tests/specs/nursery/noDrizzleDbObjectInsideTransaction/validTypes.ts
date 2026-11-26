/* should not generate diagnostics */

import { db } from "./db";

// type positions don't run queries
await db.transaction(async (tx: typeof db) => {
	const client: typeof db = tx;
	await client.select().from(users);
});
