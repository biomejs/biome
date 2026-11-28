/* should generate diagnostics */
const apiKey = process.env.GIPHY_KEY;
const computed = process.env["GIPHY_KEY"];
const template = process.env[`GIPHY_KEY`];
const optional = process?.env.GIPHY_KEY;
const computedEnv = process["env"].GIPHY_KEY;
const global = globalThis.process.env.GIPHY_KEY;
const nonNull = process.env.GIPHY_KEY!;
const { GIPHY_KEY, OTHER_KEY: renamed } = process.env;
const { ...rest } = process.env;
const name = "GIPHY_KEY";
const dynamic = process.env[name];
const keys = Object.keys(process.env);
const copy = { ...process.env };
process.env.GIPHY_KEY = "value";
delete process.env.GIPHY_KEY;
