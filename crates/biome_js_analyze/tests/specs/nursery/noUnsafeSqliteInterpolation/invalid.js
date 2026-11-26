/* should generate diagnostics */
import { DatabaseSync } from "node:sqlite";
import { DatabaseSync as Database } from "node:sqlite";
import * as sqlite from "node:sqlite";
import sqliteDefault from "node:sqlite";
import { "DatabaseSync" as QuotedDatabase } from "node:sqlite";
import { "default" as quotedSqlite } from "node:sqlite";

{
	const database = new DatabaseSync(":memory:");
	database.exec(`SELECT * FROM users WHERE id = ${id}`);
}

{
	const database = new Database(":memory:");
	database.prepare(`SELECT * FROM users WHERE id = ${id}`);
}

{
	const database = new sqlite.DatabaseSync(":memory:");
	database.exec(`SELECT * FROM users WHERE id = ${id}`);
}

{
	const database = new sqliteDefault.DatabaseSync(":memory:");
	database.prepare(`SELECT * FROM users WHERE id = ${id}`);
}

new DatabaseSync(":memory:").exec(`SELECT * FROM users WHERE id = ${id}`);

{
	const database = new DatabaseSync(":memory:");
	const query = `SELECT * FROM users WHERE id = ${id}`;
	const alias = query;
	database.prepare(alias);
}

{
	const database = new DatabaseSync(":memory:");
	database?.exec(`SELECT * FROM users WHERE id = ${id}`);
	database.exec?.(`SELECT * FROM users WHERE id = ${id}`);
	database?.["exec"](`SELECT * FROM users WHERE id = ${id}`);
	database["prepare"]?.(`SELECT * FROM users WHERE id = ${id}`);
	database["prepare"](`SELECT * FROM users WHERE id = ${id}`);
	database.exec(`SELECT ${first} FROM users WHERE id = ${second}`);
	database.exec(/* keep */ `SELECT * FROM users WHERE id = ${id}`);
	(database).exec((`SELECT * FROM users WHERE id = ${id}`));
}

{
	const database = new DatabaseSync(":memory:");
	database.exec(String.raw`SELECT * FROM users WHERE id = ${id}`);
	database.prepare(String.raw`SELECT 1`);
	database.prepare(customTag`SELECT ${id}`);
}

{
	const database = new DatabaseSync(":memory:");
	const sql = String.raw`SELECT ${id}`;
	database.exec(sql);
}

{
	const database = new DatabaseSync(":memory:");
	const sql = `SELECT ${id}`;
	database.exec(sql);
	database.prepare(sql);
}

{
	const database = new DatabaseSync(":memory:");
	const alias = database;
	alias.exec(`SELECT ${id}`);
}

{
	const database = new DatabaseSync(":memory:");
	const method = "exec";
	database[method](`SELECT ${id}`);
}

{
	const Alias = DatabaseSync;
	const database = new Alias(":memory:");
	database.exec(`SELECT ${id}`);
}

{
	const alias = sqlite;
	const database = new alias.DatabaseSync(":memory:");
	database.exec(`SELECT ${id}`);
}

{
	const Alias = sqlite.DatabaseSync;
	const database = new Alias(":memory:");
	database.exec(`SELECT ${id}`);
}

{
	const database = new sqlite["DatabaseSync"](":memory:");
	database.exec(`SELECT ${id}`);
}

{
	const database = new QuotedDatabase(":memory:");
	database.exec(`SELECT ${id}`);
}

{
	const database = new quotedSqlite.DatabaseSync(":memory:");
	database.exec(`SELECT ${id}`);
}

function query() {
	const database = new DatabaseSync(":memory:");
	database.exec(`SELECT ${id}`);
}
