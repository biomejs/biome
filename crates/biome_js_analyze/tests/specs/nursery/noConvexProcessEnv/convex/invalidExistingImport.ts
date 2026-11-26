/* should generate diagnostics */
import { query } from "./_generated/server";

export const apiKey = query(() => process.env.GIPHY_KEY);
