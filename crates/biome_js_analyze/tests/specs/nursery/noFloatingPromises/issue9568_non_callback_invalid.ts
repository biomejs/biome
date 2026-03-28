/* should generate diagnostics */

export function schedule(mode: "sync", cb: () => Promise<void>): void;
export function schedule(mode: "async", cb: () => Promise<void>): Promise<void>;
export function schedule(
	mode: "sync" | "async",
	cb: () => Promise<void>,
) {
	return cb();
}

schedule("async", async () => {
	await Promise.resolve();
});
