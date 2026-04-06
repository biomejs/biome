/* should generate diagnostics */

import { doWork, doWorkTwo } from "./wrapper";

async function main() {
	doWork("abc");
	doWorkTwo();
}
