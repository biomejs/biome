/* should not generate diagnostics */
import { env } from "./_generated/server";

const apiKey = env.GIPHY_KEY;
const other = process.version;
const argv = process.argv;

function shadowed(process: { env: Record<string, string> }) {
	return process.env.GIPHY_KEY;
}

const local = { env: {} };
const fromLocal = local.env;
