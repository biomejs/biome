/* should not generate diagnostics */

export function schedule(mode: "sync"): void;
export function schedule(mode: "async"): Promise<void>;
export function schedule(mode: "sync" | "async") {
	return Promise.resolve();
}

schedule("sync");
