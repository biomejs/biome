/* should generate diagnostics */
import process from "process";

const name = "FOO";
delete process.env[name];
