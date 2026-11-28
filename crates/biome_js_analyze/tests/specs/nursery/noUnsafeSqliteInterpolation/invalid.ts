/* should generate diagnostics */
import { DatabaseSync } from "node:sqlite";
import * as sqlite from "node:sqlite";

{
	const database = new DatabaseSync(":memory:");
	database.exec(`SELECT ${id}` as string);
	database.exec(<string>`SELECT ${id}`);
	database.exec<string>(`SELECT ${id}`);
	database.exec(`SELECT ${id}`!);
	database!.exec(`SELECT ${id}`);
	database.exec(`SELECT ${id}` satisfies string);
	(database as DatabaseSync).exec(`SELECT ${id}`);
}

{
	const database = new (DatabaseSync<string>)(":memory:");
	database.exec(`SELECT ${id}`);
}

{
	const database = new (sqlite as typeof import("node:sqlite")).DatabaseSync(":memory:");
	database.exec(`SELECT ${id}`);
}

function typeShadowsClass() {
	type DatabaseSync = {};
	const database = new DatabaseSync(":memory:");
	database.exec(`SELECT ${id}`);
}

function typeShadowsModule() {
	type sqlite = {};
	const database = new sqlite.DatabaseSync(":memory:");
	database.exec(`SELECT ${id}`);
}

const database = new DatabaseSync(":memory:");
function typeShadowsDatabase() {
	type database = string;
	database.exec(`SELECT ${id}`);
}
