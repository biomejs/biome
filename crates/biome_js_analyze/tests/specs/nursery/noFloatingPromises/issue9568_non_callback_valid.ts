/* should not generate diagnostics */

export function schedule(mode: "sync", cb: () => Promise<void>): void;
export function schedule(mode: "async", cb: () => Promise<void>): Promise<void>;
export function schedule(
	mode: "sync" | "async",
	cb: () => Promise<void>,
) {
	return cb();
}

schedule("sync", async () => {
	await Promise.resolve();
});
