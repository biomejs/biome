/* should not generate diagnostics */

import { doWork, syncFn } from "./wrapper";

async function main() {
	await doWork("abc");
	void doWork("xyz");
	doWork("123").catch(() => {});
	syncFn();
}
