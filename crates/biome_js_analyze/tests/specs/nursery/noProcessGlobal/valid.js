/* should not generate diagnostics */
import process from "node:process";

const a = process.env;

const foo = process.env.FOO;

// allow other globals
const bar = console;