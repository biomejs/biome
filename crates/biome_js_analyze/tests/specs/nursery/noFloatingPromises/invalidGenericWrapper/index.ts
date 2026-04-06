/* should generate diagnostics */

import { doWork, doWorkTwo, maybeDoWork } from "./wrapper";

async function main() {
	doWork("abc");
	doWorkTwo();
	maybeDoWork?.("xyz");
}
