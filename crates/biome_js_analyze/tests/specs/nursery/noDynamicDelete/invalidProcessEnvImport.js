/* should generate diagnostics */
import process from "node:process";

const name = "FOO";
delete process.env[name];
