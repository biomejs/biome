/* should not generate diagnostics */
import { DatabaseSync } from "node:sqlite";
import { DatabaseSync as OtherDatabaseSync } from "other-sqlite";
import * as sqlite from "node:sqlite";
import { Database } from "node:sqlite";

{
	const database = new DatabaseSync(":memory:");
	database.exec("SELECT 1");
	database.exec(`SELECT 1`);
	database.prepare("SELECT ?").get(value);
	database.run(`SELECT ${value}`);
	database.query(`SELECT ${value}`);
	database.exec("SELECT " + value);
	database.exec();
	database.exec(...queries);
	database.prepare("SELECT ?", `${value}`);
}

{
	const database = new DatabaseSync(":memory:");
	const sql = `SELECT 1`;
	database.prepare(sql);
}

{
	const database = new DatabaseSync(":memory:");
	const sql = `SELECT ${value}`;
	let alias = sql;
	database.prepare(alias);
}

{
	const database = new DatabaseSync(":memory:");
	const sql = `SELECT ${value}`;
	let alias;
	alias = sql;
	database.prepare(alias);
}

{
	const database = new DatabaseSync(":memory:");
	const { exec } = database;
	exec(`SELECT ${value}`);
}

{
	const { database } = new DatabaseSync(":memory:");
	database.exec(`SELECT ${value}`);
}

{
	const database = new DatabaseSync(":memory:");
	const method = getMethod();
	database[method](`SELECT ${value}`);
}

{
	const sql = new DatabaseSync(":memory:").createTagStore();
	sql.get`SELECT * FROM users WHERE id = ${value}`;
}

{
	const database = createDatabase();
	database.exec(`SELECT ${value}`);
}

{
	let Alias = DatabaseSync;
	const database = new Alias(":memory:");
	database.exec(`SELECT ${value}`);
}

{
	let Alias;
	Alias = DatabaseSync;
	const database = new Alias(":memory:");
	database.exec(`SELECT ${value}`);
}

{
	let database = new DatabaseSync(":memory:");
	database.exec(`SELECT ${value}`);
}

{
	var database = new DatabaseSync(":memory:");
	database.exec(`SELECT ${value}`);
}

{
	const database = new OtherDatabaseSync(":memory:");
	database.exec(`SELECT ${value}`);
}

{
	const database = new Database(":memory:");
	database.exec(`SELECT ${value}`);
}

{
	const database = new sqlite.Database(":memory:");
	database.exec(`SELECT ${value}`);
}

{
	const DatabaseSync = class {};
	const database = new DatabaseSync();
	database.exec(`SELECT ${value}`);
}

{
	const sqlite = {};
	const database = new sqlite.DatabaseSync();
	database.exec(`SELECT ${value}`);
}

for (const Database of values) {
	const database = new Database(":memory:");
	database.exec(`SELECT ${value}`);
}

unknownDatabase.exec(`SELECT ${value}`);

{
	const first = second;
	const second = first;
	first.exec(`SELECT ${value}`);
}

{
	const database = new DatabaseSync(":memory:");
	const first = second;
	const second = first;
	database.exec(first);
}
