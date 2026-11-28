/* should not generate diagnostics */
class DatabaseSync {
	exec() {}
}
const database = new DatabaseSync();
database.exec(`SELECT ${value}`);
