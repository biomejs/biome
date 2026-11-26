/* should not generate diagnostics */
import { DatabaseSync } from "node:sqlite";

declare const database: DatabaseSync;
database.exec(`SELECT ${value}`);

type other = {};
other.exec(`SELECT ${value}`);
