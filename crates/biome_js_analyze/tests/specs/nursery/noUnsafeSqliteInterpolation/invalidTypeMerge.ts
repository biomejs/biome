/* should generate diagnostics */
import { DatabaseSync } from "node:sqlite";
import * as sqlite from "node:sqlite";
type DatabaseSync = {};
type sqlite = {};

new DatabaseSync(":memory:").exec(`SELECT ${id}`);
new sqlite.DatabaseSync(":memory:").exec(`SELECT ${id}`);
