/* should not generate diagnostics */

import { trace } from "./trace";

async function _doWork(id: string): Promise<void> {
	await new Promise((resolve) => setTimeout(resolve, 100));
}

export const doWork = trace(_doWork);

function _syncFn(): string {
	return "hello";
}

export const syncFn = trace(_syncFn);
