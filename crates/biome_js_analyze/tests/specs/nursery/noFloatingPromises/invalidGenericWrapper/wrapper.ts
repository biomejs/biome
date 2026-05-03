/* should not generate diagnostics */

import { trace } from "./trace";

async function _doWork(id: string): Promise<void> {
	await new Promise((resolve) => setTimeout(resolve, 100));
}

export const doWork = trace(_doWork);

async function _doWorkTwo(): Promise<string> {
	return "value";
}

export const doWorkTwo = trace(_doWorkTwo, { name: "doWorkTwo" });

export const maybeDoWork: typeof doWork | undefined = doWork;
