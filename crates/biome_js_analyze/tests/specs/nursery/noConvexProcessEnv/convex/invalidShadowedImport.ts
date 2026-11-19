/* should generate diagnostics */
import { env } from "./_generated/server";

function apiKey(env: unknown) {
	return process.env.GIPHY_KEY;
}
