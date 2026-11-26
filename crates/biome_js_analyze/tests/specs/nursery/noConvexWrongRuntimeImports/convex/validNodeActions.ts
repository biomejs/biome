/* should not generate diagnostics */
"use node";

import { createHash } from "node:crypto";

export type Hash = string;

export function hash(value: string): Hash {
	return createHash("sha256").update(value).digest("hex");
}
